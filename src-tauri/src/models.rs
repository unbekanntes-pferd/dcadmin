use std::{sync::Arc, time::Duration};

use dco3::{
    auth::{Connected, Disconnected},
    public::SoftwareVersionData,
    user::UserAccount,
    Dracoon, FilterOperator, FilterQueryBuilder, ListAllParams, OAuth2Flow,
};
use keyring::Entry;
use moka::future::Cache;
use serde::{Deserialize, Serialize};
use tauri::async_runtime::RwLock;

use crate::{
    config::{log_dracoon_error, setup_cache},
    customer::SerializedCustomerInfo,
    events::{EventsCacheKey, SerializedEventList, SerializedOperationTypes},
    permissions::{PermissionsCacheKey, SerializedNodePermissionsList},
};

pub const ROLE_ROOM_MANAGER: &str = "ROOM_MANAGER";
pub const ROLE_CONFIG_MANAGER: &str = "CONFIG_MANAGER";
pub const ROLE_AUDITOR: &str = "LOG_AUDITOR";
pub const ROLE_USER_MANAGER: &str = "USER_MANAGER";
pub const ROLE_GROUP_MANAGER: &str = "GROUP_MANAGER";
pub const ROLE_GUEST_USER: &str = "GUEST_USER";
const DEFAULT_MAX_CACHE_ENTITY_COUNT: u64 = 100;
const DEFAULT_MAX_CACHE_STATIC_COUNT: u64 = 1;

pub enum AppAuth {
    Unset,
    AuthCode,
    RefreshToken(String),
}

pub struct AppState {
    client: Arc<RwLock<WrappedClient>>,
    app_auth: Arc<RwLock<AppAuth>>,
    entry: Arc<RwLock<Option<Entry>>>,
    cache: AppCache,
}

pub struct AppCache {
    pub permissions: Cache<PermissionsCacheKey, Arc<SerializedNodePermissionsList>>,
    pub customer: Cache<String, Arc<SerializedCustomerInfo>>,
    pub events: Cache<EventsCacheKey, Arc<SerializedEventList>>,
    pub operations: Cache<String, Arc<SerializedOperationTypes>>,
}

impl AppCache {
    pub fn new() -> Self {
        AppCache {
            permissions: setup_cache(DEFAULT_MAX_CACHE_ENTITY_COUNT, None),
            customer: setup_cache(
                DEFAULT_MAX_CACHE_STATIC_COUNT,
                Some(Duration::from_secs(30 * 60)),
            ),
            events: setup_cache(
                DEFAULT_MAX_CACHE_ENTITY_COUNT,
                Some(Duration::from_secs(60)),
            ),
            operations: setup_cache(
                DEFAULT_MAX_CACHE_STATIC_COUNT,
                Some(Duration::from_secs(30 * 60)),
            ),
        }
    }

    pub fn permissions(&self) -> &Cache<PermissionsCacheKey, Arc<SerializedNodePermissionsList>> {
        &self.permissions
    }

    pub fn customer(&self) -> &Cache<String, Arc<SerializedCustomerInfo>> {
        &self.customer
    }

    pub fn events(&self) -> &Cache<EventsCacheKey, Arc<SerializedEventList>> {
        &self.events
    }

    pub fn operations(&self) -> &Cache<String, Arc<SerializedOperationTypes>> {
        &self.operations
    }
}

pub enum WrappedClient {
    Unset,
    Connected(Dracoon<Connected>),
    Disconnected(Dracoon<Disconnected>),
}

impl Default for AppState {
    fn default() -> Self {
        AppState {
            client: Arc::new(RwLock::new(WrappedClient::Unset)),
            app_auth: Arc::new(RwLock::new(AppAuth::Unset)),
            entry: Arc::new(RwLock::new(None)),
            cache: AppCache::new(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SerializedUserAccount {
    pub first_name: String,
    pub last_name: String,
    pub is_room_manager: bool,
    pub is_config_manager: bool,
    pub is_auditor: bool,
    pub is_user_manager: bool,
    pub is_group_manager: bool,
    pub email: String,
    pub user_id: u64,
    pub is_cloud: bool,
}

impl From<(UserAccount, SoftwareVersionData)> for SerializedUserAccount {
    fn from(value: (UserAccount, SoftwareVersionData)) -> Self {
        let (account, version_data) = value;
        let roles = account.user_roles;

        let is_room_manager = roles
            .items
            .iter()
            .any(|role| role.name == ROLE_ROOM_MANAGER);
        let is_config_manager = roles
            .items
            .iter()
            .any(|role| role.name == ROLE_CONFIG_MANAGER);
        let is_auditor = roles.items.iter().any(|role| role.name == ROLE_AUDITOR);
        let is_user_manager = roles
            .items
            .iter()
            .any(|role| role.name == ROLE_USER_MANAGER);
        let is_group_manager = roles
            .items
            .iter()
            .any(|role| role.name == ROLE_GROUP_MANAGER);

        let is_cloud = if let Some(is_cloud) = version_data.is_dracoon_cloud {
            is_cloud
        } else {
            false
        };

        SerializedUserAccount {
            first_name: account.first_name,
            last_name: account.last_name,
            is_room_manager,
            is_config_manager,
            is_auditor,
            is_user_manager,
            is_group_manager,
            email: account.email.unwrap_or_default(),
            user_id: account.id,
            is_cloud,
        }
    }
}

impl AppState {
    pub fn entry(&self) -> &Arc<RwLock<Option<Entry>>> {
        &self.entry
    }

    pub async fn set_entry(&self, entry: Entry) {
        *self.entry.write().await = Some(entry);
    }

    pub fn app_auth(&self) -> Arc<RwLock<AppAuth>> {
        self.app_auth.clone()
    }

    pub async fn set_refresh_token(&self, refresh_token: String) {
        *self.app_auth.write().await = AppAuth::RefreshToken(refresh_token);
    }

    pub async fn set_auth_code(&self) {
        *self.app_auth.write().await = AppAuth::AuthCode;
    }

    pub async fn init_client(&self, client: Dracoon<Disconnected>) {
        *self.client.write().await = WrappedClient::Disconnected(client);
    }

    pub async fn connect(&self, auth_code: String) -> Result<(UserAccount, String), String> {
        let read_lock = self.client.read().await;

        let client_clone = match &*read_lock {
            WrappedClient::Disconnected(client) => client.clone(),
            WrappedClient::Connected(client) => {
                return Ok((
                    client.get_user_info().await.map_err(|e| {
                        log_dracoon_error(&e, Some("Error fetching user info"));
                        e.to_string()
                    })?,
                    client.get_refresh_token().await.to_string(),
                ))
            }
            WrappedClient::Unset => return Err("Client not initialized".to_string()),
        };
        // release read lock
        drop(read_lock);

        let client = client_clone
            .connect(OAuth2Flow::authorization_code(auth_code))
            .await
            .map_err(|e| {
                tracing::error!("Error connecting with auth code: {e}");
                e.to_string()
            })?;

        let refresh_token = client.get_refresh_token().await.to_string();

        *self.client.write().await = WrappedClient::Connected(client);

        let user_info = self.get_user_info().await?;

        Ok((user_info, refresh_token))
    }

    pub async fn connect_refresh_token(
        &self,
        refresh_token: impl Into<String>,
    ) -> Result<(UserAccount, String), String> {
        let read_lock = self.client.read().await;

        let client_clone = match &*read_lock {
            WrappedClient::Disconnected(client) => client.clone(),
            WrappedClient::Connected(client) => {
                let refresh_token = refresh_token.into();
                return Ok((
                    client.get_user_info().await.map_err(|e| e.to_string())?,
                    refresh_token,
                ));
            }
            WrappedClient::Unset => return Err("Client not initialized".to_string()),
        };
        // release read lock
        drop(read_lock);

        let client = client_clone
            .connect(OAuth2Flow::refresh_token(refresh_token))
            .await
            .map_err(|e| {
                tracing::error!("Error connecting with refresh token: {e}");
                e.to_string()
            })?;

        let refresh_token = client.get_refresh_token().await.to_string();

        *self.client.write().await = WrappedClient::Connected(client);

        let user_info = self.get_user_info().await?;

        Ok((user_info, refresh_token))
    }

    async fn get_user_info(&self) -> Result<UserAccount, String> {
        let client = self.get_client().await?;

        let user_info = client.get_user_info().await.map_err(|e| e.to_string())?;

        Ok(user_info)
    }

    pub async fn get_client(&self) -> Result<Dracoon<Connected>, String> {
        let read_lock = self.client.read().await;

        let client = match &*read_lock {
            WrappedClient::Connected(client) => client.clone(),
            WrappedClient::Disconnected(_) => return Err("Client not connected".to_string()),
            WrappedClient::Unset => return Err("Client not initialized".to_string()),
        };

        Ok(client)
    }

    pub fn get_permissions_cache(
        &self,
    ) -> &Cache<PermissionsCacheKey, Arc<SerializedNodePermissionsList>> {
        &self.cache.permissions()
    }

    pub fn get_customer_cache(&self) -> &Cache<String, Arc<SerializedCustomerInfo>> {
        &self.cache.customer()
    }

    pub fn get_events_cache(&self) -> &Cache<EventsCacheKey, Arc<SerializedEventList>> {
        &self.cache.events()
    }

    pub fn get_operations_cache(&self) -> &Cache<String, Arc<SerializedOperationTypes>> {
        &self.cache.operations()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Default)]
pub struct ListParams {
    pub offset: Option<u64>,
    pub limit: Option<u64>,
    pub filter: Option<String>,
    pub sort: Option<String>,
}

impl ListParams {
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

impl TryFrom<ListParams> for ListAllParams {
    type Error = String;

    fn try_from(value: ListParams) -> Result<Self, Self::Error> {
        let filter = value
            .filter
            .map(|filter| {
                let filters = filter.split('|').collect::<Vec<&str>>();

                let mut filter_results = Vec::new();

                for filter in filters {
                    let filter_parts = filter.split(':').collect::<Vec<&str>>();

                    let field = filter_parts.first().ok_or("Filter field not found")?;
                    let operator = filter_parts.get(1).ok_or("Filter operator not found")?;
                    let operator = match *operator {
                        "eq" => FilterOperator::Eq,
                        "neq" => FilterOperator::Neq,
                        "le" => FilterOperator::Le,
                        "ge" => FilterOperator::Ge,
                        "cn" => FilterOperator::Cn,
                        _ => return Err("Invalid filter operator".to_string()),
                    };
                    let value = filter_parts.get(2).ok_or("Filter value not found")?;

                    let filter = FilterQueryBuilder::new()
                        .with_field(*field)
                        .with_operator(operator)
                        .with_value(*value)
                        .try_build()
                        .map_err(|e| e.to_string())?;

                    filter_results.push(filter);
                }

                Ok(filter_results)
            })
            .and_then(|filter| filter.ok());

        let mut list_params = ListAllParams::builder();

        if let Some(offset) = value.offset {
            list_params = list_params.with_offset(offset);
        }

        if let Some(limit) = value.limit {
            list_params = list_params.with_limit(limit);
        }

        if let Some(filters) = filter {
            for filter in filters {
                list_params = list_params.with_filter(filter);
            }
        }

        Ok(list_params.build())
    }
}

#[derive(Serialize, Clone)]
pub struct Range {
    pub offset: u64,
    pub limit: u64,
    pub total: u64,
}

impl From<dco3::Range> for Range {
    fn from(value: dco3::Range) -> Self {
        Range {
            offset: value.offset,
            limit: value.limit,
            total: value.total,
        }
    }
}