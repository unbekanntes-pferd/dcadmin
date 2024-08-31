use credentials::HandleCredentials;
use dco3::{Dracoon, Public};
use keyring::Entry;
use tauri::State;

mod credentials;
use crate::models::{AppAuth, AppState, SerializedUserAccount};

const SERVICE_NAME: &str = "dcadmin";

#[tauri::command]
pub async fn init_auth_code_flow(url: String, state: State<'_, AppState>) -> Result<bool, String> {
    let (dcadmin_client_id, dcadmin_client_secret) = crate::config::get_client_credentials();

    let url_without_https = url.trim_start_matches("https://");

    let entry = Entry::new(SERVICE_NAME, &url_without_https).map_err(|e| e.to_string())?;

    let client = Dracoon::builder()
        .with_base_url(url)
        .with_client_id(dcadmin_client_id)
        .with_client_secret(dcadmin_client_secret)
        .build()
        .map_err(|e| e.to_string())?;

    if let Ok(refresh_token) = entry.get_dracoon_env() {
        state.init_client(client).await;
        state.set_refresh_token(refresh_token).await;
        state.set_entry(entry).await;
        return Ok(true);
    }

    let authorize_url = client.get_authorize_url();

    open::that(authorize_url.clone()).map_err(|e| e.to_string())?;

    state.init_client(client).await;
    state.set_auth_code().await;
    state.set_entry(entry).await;

    Ok(false)
}

#[tauri::command]
pub async fn connect(
    auth_code: Option<String>,
    state: State<'_, AppState>,
) -> Result<SerializedUserAccount, String> {
    let user_account = match &*state.app_auth().read().await {
        AppAuth::RefreshToken(refresh_token) => {
            // TODO: handle connection error and delete refresh token
            let (user_account, new_refresh_token) = state
                .connect_refresh_token(refresh_token.to_string())
                .await?;

            let entry = state.entry().read().await;

            if let Some(entry) = &*entry {
                entry
                    .set_dracoon_env(&new_refresh_token)
                    .map_err(|e| e.to_string())?;
            }

            user_account
        }
        AppAuth::AuthCode => {
            if let Some(auth_code) = auth_code {
                let (user_account, refresh_token) = state.connect(auth_code).await?;
                let entry = state.entry().read().await;

                if let Some(entry) = &*entry {

                    entry
                        .set_dracoon_env(&refresh_token)
                        .map_err(|e| e.to_string())?;
                }

                user_account
            } else {
                return Err("No auth code provided".to_string());
            }
        }
        AppAuth::Unset => return Err("No auth code or refresh token provided".to_string()),
    };

    let client = state.get_client().await?;
    let version_data = client
        .public()
        .get_software_version()
        .await
        .map_err(|e| e.to_string())?;

    Ok((user_account, version_data).into())
}

#[tauri::command]
pub async fn validate_dracoon_url(url: String) -> Result<bool, String> {
    let (dcadmin_client_id, dcadmin_client_secret) = crate::config::get_client_credentials();

    let client = Dracoon::builder()
        .with_base_url(url)
        .with_client_id(dcadmin_client_id)
        .with_client_secret(dcadmin_client_secret)
        .build()
        .map_err(|e| e.to_string())?;

    let result = client
        .public()
        .get_software_version()
        .await
        .map(|_| true)
        .map_err(|e| e.to_string());

    if result.is_err() {
        return Ok(false);
    }

    result
}
