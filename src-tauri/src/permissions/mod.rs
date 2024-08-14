mod models;
use crate::{models::ListParams, AppState};
use dco3::{eventlog::AuditNodesFilter, Eventlog, ListAllParams, Users};
use models::{AuditNodeListWrapper, FlattenedNodePermissions, SerializedNodePermissionsList};
use tauri::State;

#[tauri::command]
#[allow(deprecated)]
pub async fn get_permissions(
    params: ListParams,
    state: State<'_, AppState>,
) -> Result<SerializedNodePermissionsList, String> {
    let client = state.get_client().await?;

    let events = client
        .eventlog
        .get_node_permissions(params.try_into()?)
        .await
        .map_err(|e| e.to_string())?;

    let wrapped_events: AuditNodeListWrapper = events.into();

    Ok(wrapped_events.into())
}

#[tauri::command]
#[allow(deprecated)]
pub async fn export_user_permissions(
    params: ListParams,
    path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let client = state.get_client().await?;

    let permissions = client
        .eventlog
        .get_node_permissions(params.try_into()?)
        .await
        .map_err(|e| e.to_string())?;

    let wrapped_permissions: AuditNodeListWrapper = permissions.into();
    let serializable_permissions: SerializedNodePermissionsList = wrapped_permissions.into();
    let flattened_permissions: Vec<FlattenedNodePermissions> = serializable_permissions
        .into_iter()
        .map(|perms| Into::<Vec<FlattenedNodePermissions>>::into(perms))
        .flatten()
        .collect();

    let mut csv_writer = csv::Writer::from_path(path).map_err(|e| e.to_string())?;

    for event in flattened_permissions {
        csv_writer.serialize(event).map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
#[allow(deprecated)]
pub async fn export_all_user_permissions(
    path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut users = state
        .get_client()
        .await?
        .users
        .get_users(None, None, None)
        .await
        .map_err(|e| e.to_string())?;

    if users.range.total > 500 {
        for offset in (500..users.range.total).step_by(500) {
            let params = ListAllParams::builder().with_offset(offset).build();
            let new_users = state
                .get_client()
                .await?
                .users
                .get_users(Some(params), None, None)
                .await
                .map_err(|e| e.to_string())?;

            users.items.extend(new_users.items);
        }
    }

    let user_ids = users
        .items
        .into_iter()
        .map(|user| user.id)
        .collect::<Vec<_>>();

    let mut node_permissions = Vec::new();

    for user_id in user_ids {
        let user_filter = AuditNodesFilter::user_id_equals(user_id);
        let params = ListAllParams::builder().with_filter(user_filter).build();

        let permissions = state
            .get_client()
            .await?
            .eventlog
            .get_node_permissions(params)
            .await
            .map_err(|e| e.to_string())?;

        node_permissions.extend(permissions);
    }

    let wrapped_permissions: AuditNodeListWrapper = node_permissions.into();
    let serializable_permissions: SerializedNodePermissionsList = wrapped_permissions.into();
    let flattened_permissions: Vec<FlattenedNodePermissions> = serializable_permissions
        .into_iter()
        .map(|perms| Into::<Vec<FlattenedNodePermissions>>::into(perms))
        .flatten()
        .collect();

    let mut csv_writer = csv::Writer::from_path(path).map_err(|e| e.to_string())?;

    for event in flattened_permissions {
        csv_writer.serialize(event).map_err(|e| e.to_string())?;
    }

    Ok(())
}
