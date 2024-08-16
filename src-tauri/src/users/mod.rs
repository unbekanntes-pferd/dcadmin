use crate::{models::ListParams, AppState};
use dco3::Users;
use models::{FlattenedUserItem, SerializedUserItem, SerializedUserList};
use tauri::State;

mod models;

#[tauri::command]
pub async fn get_users(
    state: State<'_, AppState>,
    params: ListParams,
) -> Result<SerializedUserList, String> {
    let client = state.get_client().await?;
    let params = params.try_into()?;

    Ok(client
        .users
        .get_users(Some(params), Some(true), None)
        .await
        .map_err(|e| e.to_string())?
        .into())
}

#[tauri::command]
pub async fn export_users(
    params: ListParams,
    path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let client = state.get_client().await?;

    let mut users = client
        .users
        .get_users(Some(params.clone().try_into()?), Some(true), None)
        .await
        .map_err(|e| e.to_string())?;

    for offset in (500..users.range.total).step_by(500) {
        let params = ListParams {
            offset: Some(offset),
            ..params.clone()
        };

        let new_users = client
            .users
            .get_users(Some(params.try_into()?), Some(true), None)
            .await
            .map_err(|e| e.to_string())?;

        users.items.extend(new_users.items);
    }

    let serialized_users: Vec<SerializedUserItem> = users
        .items
        .into_iter()
        .map(|user| user.into())
        .collect::<Vec<_>>();
    let flattened_users: Vec<FlattenedUserItem> = serialized_users
        .into_iter()
        .map(|user| user.into())
        .collect::<Vec<_>>();

    let mut csv_writer = csv::Writer::from_path(path).map_err(|e| e.to_string())?;

    for user in flattened_users {
        csv_writer.serialize(user).map_err(|e| e.to_string())?;
    }

    csv_writer.flush().map_err(|e| e.to_string())?;

    Ok(())
}
