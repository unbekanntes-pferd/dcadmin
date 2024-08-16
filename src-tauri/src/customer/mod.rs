use std::sync::Arc;

use crate::AppState;
use dco3::User;
pub use models::SerializedCustomerInfo;
use tauri::State;

mod models;

#[tauri::command]
pub async fn get_customer_info(
    state: State<'_, AppState>,
) -> Result<SerializedCustomerInfo, String> {
    let client = state.get_client().await?;

    if let Some(info) = state
        .get_customer_cache()
        .get(&client.get_base_url().to_string())
        .await
    {
        return Ok((*info).clone());
    }

    let info = client
        .user
        .get_customer_info()
        .await
        .map_err(|e| e.to_string())?;

    let serializable_info: SerializedCustomerInfo = info.into();
    let serializable_info = Arc::new(serializable_info);

    state
        .get_customer_cache()
        .insert(client.get_base_url().to_string(), serializable_info.clone())
        .await;

    Ok((*serializable_info).clone())
}
