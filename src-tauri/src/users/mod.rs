use std::time::Instant;

use crate::{config::log_dracoon_error, models::ListParams, AppState};
use dco3::Users;
use models::{FlattenedUserItem, SerializedUserItem, SerializedUserList};
use tauri::State;

pub (crate) use models::SerializedRoleList;

mod models;

#[tauri::command]
pub async fn get_users(
    state: State<'_, AppState>,
    params: ListParams,
) -> Result<SerializedUserList, String> {
    let now = Instant::now();
    let client = state.get_client().await?;
    let params = params.try_into()?;

    let users = client
        .users()
        .get_users(Some(params), Some(true), None)
        .await
        .map_err(|e| {
            log_dracoon_error(&e, Some("Error fetching users"));
            e.to_string()
        })?;

    let user_count = users.items.len();

    let elapsed = now.elapsed().as_millis();
    tracing::info!("Fetched {user_count} users in {elapsed} ms");
    Ok(users.into())
}

#[tauri::command]
pub async fn export_users(
    params: ListParams,
    path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let now = Instant::now();
    let client = state.get_client().await?;

    let mut users = client
        .users()
        .get_users(Some(params.clone().try_into()?), Some(true), None)
        .await
        .map_err(|e| {
            log_dracoon_error(&e, Some("Error fetching users"));
            e.to_string()
        })?;

    let elapsed_first_500_users = now.elapsed().as_millis();
    tracing::info!("Fetched first 500 users in {elapsed_first_500_users} ms");

    for offset in (500..users.range.total).step_by(500) {
        tracing::debug!("Fetching users with offset {offset}");
        let params = ListParams {
            offset: Some(offset),
            ..params.clone()
        };

        let new_users = client
            .users()
            .get_users(Some(params.try_into()?), Some(true), None)
            .await
            .map_err(|e| {
                log_dracoon_error(&e, Some("Error fetching users"));
                e.to_string()
            })?;

        users.items.extend(new_users.items);
    }

    let elapsed_all_users = now.elapsed().as_millis();
    tracing::info!("Fetched all users in {elapsed_all_users} ms");

    let serialized_users: Vec<SerializedUserItem> = users
        .items
        .into_iter()
        .map(|user| user.into())
        .collect::<Vec<_>>();
    let flattened_users: Vec<FlattenedUserItem> = serialized_users
        .into_iter()
        .map(|user| user.into())
        .collect::<Vec<_>>();

    let mut csv_writer = csv::Writer::from_path(path).map_err(|e| {
        tracing::error!("Error creating CSV writer: {e}");
        e.to_string()
    })?;

    for user in flattened_users {
        csv_writer.serialize(user).map_err(|e| {
            tracing::error!("Error serializing user info: {e}");
            e.to_string()
        })?;
    }

    csv_writer.flush().map_err(|e| {
        tracing::error!("Error flushing CSV writer: {e}");
        e.to_string()
    })?;

    let elapsed_csv = now.elapsed().as_millis();
    tracing::info!("Exported all users to CSV in {elapsed_csv} ms");

    Ok(())
}
