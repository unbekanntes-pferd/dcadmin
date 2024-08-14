use crate::AppState;
use dco3::User;
use models::SerializedCustomerInfo;
use tauri::State;

mod models;

#[tauri::command]
pub async fn get_customer_info(
    state: State<'_, AppState>,
) -> Result<SerializedCustomerInfo, String> {
    let client = state.get_client().await?;

    Ok(client
        .user
        .get_customer_info()
        .await
        .map_err(|e| e.to_string())?
        .into())
}
