use std::time::Instant;

use credentials::HandleCredentials;
use dco3::{Dracoon, Public};
use keyring::Entry;
use tauri::State;

mod credentials;
use crate::{
    config::{log_dracoon_error, APPLICATION_NAME},
    models::{AppAuth, AppState, SerializedUserAccount},
};

const SERVICE_NAME: &str = "dcadmin";

#[tauri::command]
pub async fn init_auth_code_flow(url: String, state: State<'_, AppState>) -> Result<bool, String> {
    let now = Instant::now();
    let (dcadmin_client_id, dcadmin_client_secret) = crate::config::get_client_credentials();

    let url_without_https = url.trim_start_matches("https://");

    let entry = Entry::new(SERVICE_NAME, &url_without_https).map_err(|e| {
        tracing::error!("Error creating keyring entry: {e}");
        e.to_string()
    })?;

    let client = Dracoon::builder()
        .with_base_url(url)
        .with_client_id(dcadmin_client_id)
        .with_client_secret(dcadmin_client_secret)
        .with_user_agent(APPLICATION_NAME)
        .build()
        .map_err(|e| {
            tracing::error!("Error building client: {e}");
            e.to_string()
        })?;

    if let Ok(refresh_token) = entry.get_dracoon_env() {
        tracing::info!("Found refresh token in keyring");
        state.init_client(client).await;
        state.set_refresh_token(refresh_token).await;
        state.set_entry(entry).await;
        let elapsed = now.elapsed().as_millis();
        tracing::info!("Initialized refresh token flow in {elapsed} ms");
        return Ok(true);
    }

    tracing::info!("No refresh token found in keyring");

    let authorize_url = client.get_authorize_url();

    open::that(authorize_url.clone()).map_err(|e| e.to_string())?;

    state.init_client(client).await;
    state.set_auth_code().await;
    state.set_entry(entry).await;

    let elapsed = now.elapsed().as_millis();
    tracing::info!("Initialized auth code flow in {elapsed} ms");
    Ok(false)
}

#[tauri::command]
pub async fn connect(
    auth_code: Option<String>,
    state: State<'_, AppState>,
) -> Result<SerializedUserAccount, String> {
    let now = Instant::now();
    let user_account = match &*state.app_auth().read().await {
        AppAuth::RefreshToken(refresh_token) => {
            match state.connect_refresh_token(refresh_token).await {
                Ok((user_account, new_refresh_token)) => {
                    tracing::info!("Connected with refresh token");
                    let entry = state.entry().read().await;

                    if let Some(entry) = &*entry {
                        entry.set_dracoon_env(&new_refresh_token).map_err(|e| {
                            tracing::error!("Error updating refresh token in keyring: {e}");
                            e.to_string()
                        })?;
                    }

                    tracing::info!("Updated refresh token in keyring");
                    user_account
                }
                Err(e) => {
                    tracing::error!("Error connecting with refresh token: {e}");
                    let entry = state.entry().read().await;
                    if let Some(entry) = &*entry {
                        entry.delete_dracoon_env().map_err(|e| {
                            tracing::error!(
                                "Error deleting invalid refresh token from keyring: {e}"
                            );
                            e.to_string()
                        })?;
                        tracing::info!("Deleted invalid refresh token from keyring: {e}");
                    }

                    return Err(e.to_string());
                }
            }
        }
        AppAuth::AuthCode => {
            if let Some(auth_code) = auth_code {
                tracing::info!("Connecting with auth code");
                let (user_account, refresh_token) = state.connect(auth_code).await?;
                let entry = state.entry().read().await;

                tracing::info!("Connected with auth code");

                if let Some(entry) = &*entry {
                    entry.set_dracoon_env(&refresh_token).map_err(|e| {
                        tracing::error!("Error storing refresh token in keyring: {e}");
                        e.to_string()
                    })?;
                }

                tracing::info!("Stored refresh token in keyring");
                user_account
            } else {
                tracing::error!("No auth code provided");
                return Err("No auth code provided".to_string());
            }
        }
        AppAuth::Unset => return Err("No auth code or refresh token provided".to_string()),
    };

    let client = state.get_client().await?;
    let version_data = client.public().get_software_version().await.map_err(|e| {
        log_dracoon_error(&e, Some("Error fetching software version"));
        e.to_string()
    })?;

    let elapsed = now.elapsed().as_millis();
    tracing::info!("Connected in {elapsed} ms");
    Ok((user_account, version_data).into())
}

#[tauri::command]
pub async fn validate_dracoon_url(url: String) -> Result<bool, String> {
    let now = Instant::now();
    let (dcadmin_client_id, dcadmin_client_secret) = crate::config::get_client_credentials();

    let client = Dracoon::builder()
        .with_base_url(&url)
        .with_client_id(dcadmin_client_id)
        .with_client_secret(dcadmin_client_secret)
        .with_user_agent(APPLICATION_NAME)
        .build()
        .map_err(|e| {
            tracing::error!("Error building public client: {e}");
            e.to_string()
        })?;

    let result = client
        .public()
        .get_software_version()
        .await
        .map(|_| true)
        .map_err(|e| {
            log_dracoon_error(&e, Some("Error validating Dracoon url"));
            e.to_string()
        });

    let elapsed = now.elapsed().as_millis();
    tracing::info!("Validated Dracoon url in {elapsed} ms");

    if result.is_err() {
        tracing::error!("Error validating Dracoon url - invalid url: {url}");
        return Ok(false);
    }

    result
}
