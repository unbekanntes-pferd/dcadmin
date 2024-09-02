mod models;
use std::{sync::Arc, time::Instant};

use crate::{config::log_dracoon_error, models::ListParams, AppState};
use dco3::{eventlog::AuditNodesFilter, Eventlog, ListAllParams, Users};
use models::{AuditNodeListWrapper, FlattenedNodePermissions};
use tauri::State;

pub use models::{PermissionsCacheKey, SerializedNodePermissionsList};

#[tauri::command]
#[allow(deprecated)]
pub async fn get_permissions(
    params: ListParams,
    state: State<'_, AppState>,
) -> Result<SerializedNodePermissionsList, String> {
    let now = Instant::now();
    let client = state.get_client().await?;

    let url = client.get_base_url().to_string();
    let key = PermissionsCacheKey::new(url, params.clone());

    if let Some(permissions) = state.get_permissions_cache().get(&key).await {
        let elapsed = now.elapsed().as_millis();
        tracing::info!("Fetched cached permissions in {elapsed} ms");
        return Ok((*permissions).clone());
    }

    let permissions = client
        .eventlog()
        .get_node_permissions(params.try_into()?)
        .await
        .map_err(|e| {
            log_dracoon_error(&e, Some("Error fetching permissions"));
            e.to_string()
        })?;

    let wrapped_permissions: AuditNodeListWrapper = permissions.into();
    let serializable_permissions: SerializedNodePermissionsList = wrapped_permissions.into();
    let serializable_permissions = Arc::new(serializable_permissions);

    state
        .get_permissions_cache()
        .insert(key, serializable_permissions.clone())
        .await;

    let elapsed = now.elapsed().as_millis();
    tracing::info!("Fetched permissions in {elapsed} ms");
    Ok((*serializable_permissions).clone())
}

#[tauri::command]
#[allow(deprecated)]
pub async fn export_user_permissions(
    params: ListParams,
    path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let now = Instant::now();
    let client = state.get_client().await?;

    let url = client.get_base_url().to_string();
    let key = PermissionsCacheKey::new(url, params.clone());

    let serializable_permissions = if let Some(permissions) =
        state.get_permissions_cache().get(&key).await
    {
        (*permissions).clone()
    } else {
        let fetched_permissions = client
            .eventlog()
            .get_node_permissions(params.try_into()?)
            .await
            .map_err(|e| {
                log_dracoon_error(&e, Some("Error fetching permissions"));
                e.to_string()
            })?;

        let wrapped_permissions: AuditNodeListWrapper = fetched_permissions.into();
        let serializable_permissions: SerializedNodePermissionsList = wrapped_permissions.into();
        serializable_permissions
    };
    let elapsed_fetched_events = now.elapsed().as_millis();
    tracing::info!("Fetched permissions in {elapsed_fetched_events} ms");

    let flattened_permissions: Vec<FlattenedNodePermissions> = serializable_permissions
        .into_iter()
        .map(|perms| Into::<Vec<FlattenedNodePermissions>>::into(perms))
        .flatten()
        .collect();

    let mut csv_writer = csv::Writer::from_path(path).map_err(|e| {
        tracing::error!("Error creating CSV writer: {}", e);
        e.to_string()
    })?;

    for event in flattened_permissions {
        csv_writer.serialize(event).map_err(|e| {
            tracing::error!("Error serializing event: {}", e);
            e.to_string()
        })?;
    }

    csv_writer.flush().map_err(|e| {
        tracing::error!("Error flushing CSV writer: {}", e);
        e.to_string()
    })?;

    let elapsed_exported_events = now.elapsed().as_millis();
    tracing::info!("Exported permissions in {elapsed_exported_events} ms");
    Ok(())
}

#[tauri::command]
#[allow(deprecated)]
pub async fn export_all_user_permissions(
    path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let now = Instant::now();
    let mut users = state
        .get_client()
        .await?
        .users()
        .get_users(None, None, None)
        .await
        .map_err(|e| {
            log_dracoon_error(&e, Some("Error fetching users"));
            e.to_string()
        })?;

    if users.range.total > 500 {
        for offset in (500..users.range.total).step_by(500) {
            let params = ListAllParams::builder().with_offset(offset).build();
            let new_users = state
                .get_client()
                .await?
                .users()
                .get_users(Some(params), None, None)
                .await
                .map_err(|e| {
                    log_dracoon_error(&e, Some("Error fetching users"));
                    e.to_string()
                })?;

            users.items.extend(new_users.items);
        }
    }

    let user_ids = users
        .items
        .into_iter()
        .map(|user| user.id)
        .collect::<Vec<_>>();

    let elapsed_fetched_users = now.elapsed().as_millis();
    tracing::info!("Fetched all users in {elapsed_fetched_users} ms");

    let mut node_permissions = Vec::new();

    for user_id in user_ids {
        let user_filter = AuditNodesFilter::user_id_equals(user_id);
        let params = ListAllParams::builder().with_filter(user_filter).build();

        let permissions = state
            .get_client()
            .await?
            .eventlog()
            .get_node_permissions(params)
            .await
            .map_err(|e| {
                log_dracoon_error(&e, Some("Error fetching permissions"));
                e.to_string()
            })?;

        node_permissions.extend(permissions);
    }
    let elapsed_fetched_permissions = now.elapsed().as_millis();
    tracing::info!("Fetched all permissions in {elapsed_fetched_permissions} ms");

    let wrapped_permissions: AuditNodeListWrapper = node_permissions.into();
    let serializable_permissions: SerializedNodePermissionsList = wrapped_permissions.into();
    let flattened_permissions: Vec<FlattenedNodePermissions> = serializable_permissions
        .into_iter()
        .map(|perms| Into::<Vec<FlattenedNodePermissions>>::into(perms))
        .flatten()
        .collect();

    let mut csv_writer = csv::Writer::from_path(path).map_err(|e| {
        tracing::error!("Error creating CSV writer: {e}");
        e.to_string()
    })?;

    for event in flattened_permissions {
        csv_writer.serialize(event).map_err(|e| {
            tracing::error!("Error serializing event: {e}");
            e.to_string()
        })?;
    }

    csv_writer.flush().map_err(|e| {
        tracing::error!("Error flushing CSV writer: {e}");
        e.to_string()
    })?;
    let elapsed_exported_permissions = now.elapsed().as_millis();
    tracing::info!("Exported all permissions in {elapsed_exported_permissions} ms");

    Ok(())
}
