use crate::{models::ListParams, AppState};
use dco3::Users;
use models::SerializedUserList;
use tauri::State;

mod models;

#[tauri::command]
pub async fn get_users(
    state: State<'_, AppState>,
    params: ListParams,
) -> Result<SerializedUserList, String> {
    let client = state.get_client().await?;
    eprintln!("params: {:?}", params);
    let params = params.try_into()?;
    eprintln!("params: {:?}", params);

    Ok(client
        .users
        .get_users(Some(params), Some(true), None)
        .await
        .map_err(|e| e.to_string())?
        .into())
}
