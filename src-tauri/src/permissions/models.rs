use std::hash::Hash;

use dco3::{
    eventlog::{AuditNodeList, AuditNodeResponse, AuditUserPermission},
    nodes::NodePermissions,
};
use serde::Serialize;

use crate::models::ListParams;

pub type SerializedNodePermissionsList = Vec<SerializedNodePermissions>;

#[derive(PartialEq, Eq)]
pub struct PermissionsCacheKey {
    url: String,
    params: ListParams
}

impl PermissionsCacheKey {
    pub fn new(url: String, params: ListParams) -> Self {
        Self {
            url,
            params
        }
    }
}

impl From<&PermissionsCacheKey> for String {
    fn from(value: &PermissionsCacheKey) -> Self {
        format!("{}{}", value.url, value.params.to_string())
    }
}

impl Hash for PermissionsCacheKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let key: String = self.into();
        key.hash(state);
    }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SerializedNodePermissions {
    pub node_id: i64,
    pub node_name: String,
    pub node_parent_path: String,
    pub node_cnt_children: u64,
    pub user_permissions: Vec<SerializedUserPermissions>,
    pub node_parent_id: Option<i64>,
    pub node_size: Option<u64>,
    pub node_recycle_bin_retention_period: Option<u64>,
    pub node_quota: Option<u64>,
    pub node_is_encrypted: Option<bool>,
    pub node_has_activities_log: Option<bool>,
    pub node_created_at: Option<String>,
    pub node_updated_at: Option<String>,
    pub node_created_by: Option<String>,
    pub node_created_by_id: Option<i64>,
    pub node_updated_by: Option<String>,
    pub node_updated_by_id: Option<i64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlattenedNodePermissions {
    pub node_id: i64,
    pub node_name: String,
    pub node_parent_path: String,
    pub node_cnt_children: u64,
    pub node_parent_id: Option<i64>,
    pub node_size: Option<u64>,
    pub node_recycle_bin_retention_period: Option<u64>,
    pub node_quota: Option<u64>,
    pub node_is_encrypted: Option<bool>,
    pub node_has_activities_log: Option<bool>,
    pub node_created_at: Option<String>,
    pub node_updated_at: Option<String>,
    pub node_created_by: Option<String>,
    pub node_created_by_id: Option<i64>,
    pub node_updated_by: Option<String>,
    pub node_updated_by_id: Option<i64>,
    pub user_id: i64,
    pub user_login: String,
    pub user_first_name: String,
    pub user_last_name: String,
    pub manage: bool,
    pub read: bool,
    pub create: bool,
    pub change: bool,
    pub delete: bool,
    pub manage_download_share: bool,
    pub manage_upload_share: bool,
    pub read_recycle_bin: bool,
    pub restore_recycle_bin: bool,
    pub delete_recycle_bin: bool,
}

impl From<SerializedNodePermissions> for Vec<FlattenedNodePermissions> {
    fn from(value: SerializedNodePermissions) -> Self {
        value
            .user_permissions
            .into_iter()
            .map(|user| FlattenedNodePermissions {
                node_id: value.node_id,
                node_name: value.node_name.clone(),
                node_parent_path: value.node_parent_path.clone(),
                node_cnt_children: value.node_cnt_children,
                node_parent_id: value.node_parent_id,
                node_size: value.node_size,
                node_recycle_bin_retention_period: value.node_recycle_bin_retention_period,
                node_quota: value.node_quota,
                node_is_encrypted: value.node_is_encrypted,
                node_has_activities_log: value.node_has_activities_log,
                node_created_at: value.node_created_at.clone(),
                node_updated_at: value.node_updated_at.clone(),
                node_created_by: value.node_created_by.clone(),
                node_created_by_id: value.node_created_by_id,
                node_updated_by: value.node_updated_by.clone(),
                node_updated_by_id: value.node_updated_by_id,
                user_id: user.user_id,
                user_login: user.user_login.clone(),
                user_first_name: user.user_first_name.clone(),
                user_last_name: user.user_last_name.clone(),
                manage: user.permissions.manage,
                read: user.permissions.read,
                create: user.permissions.create,
                change: user.permissions.change,
                delete: user.permissions.delete,
                manage_download_share: user.permissions.manage_download_share,
                manage_upload_share: user.permissions.manage_upload_share,
                read_recycle_bin: user.permissions.read_recycle_bin,
                restore_recycle_bin: user.permissions.restore_recycle_bin,
                delete_recycle_bin: user.permissions.delete_recycle_bin,
            })
            .collect()
    }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SerializedUserPermissions {
    pub user_id: i64,
    pub user_login: String,
    pub user_first_name: String,
    pub user_last_name: String,
    pub permissions: NodePermissions,
}

pub struct AuditNodeListWrapper(AuditNodeList);

impl From<AuditNodeList> for AuditNodeListWrapper {
    fn from(value: AuditNodeList) -> Self {
        Self(value)
    }
}

impl From<AuditNodeListWrapper> for SerializedNodePermissionsList {
    fn from(wrapper: AuditNodeListWrapper) -> Self {
        wrapper.0.into_iter().map(|item| item.into()).collect()
    }
}

impl From<AuditNodeResponse> for SerializedNodePermissions {
    fn from(value: AuditNodeResponse) -> Self {
        Self {
            node_id: value.node_id,
            node_name: value.node_name,
            node_parent_path: value.node_parent_path,
            node_cnt_children: value.node_cnt_children,
            user_permissions: value
                .audit_user_permission_list
                .into_iter()
                .map(|item| item.into())
                .collect(),
            node_parent_id: value.node_parent_id,
            node_size: value.node_size,
            node_recycle_bin_retention_period: value.node_recycle_bin_retention_period,
            node_quota: value.node_quota,
            node_is_encrypted: value.node_is_encrypted,
            node_has_activities_log: value.node_has_activities_log,
            node_created_at: value
                .node_created_at
                .map(|d| d.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)),
            node_updated_at: value
                .node_updated_at
                .map(|d| d.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)),
            node_created_by: value
                .node_created_by
                .as_ref()
                .map(|user| user.user_name.as_deref().unwrap_or_default().to_string()),
            node_created_by_id: value.node_created_by.as_ref().map(|user| user.id),
            node_updated_by: value
                .node_updated_by
                .as_ref()
                .map(|user| user.user_name.as_deref().unwrap_or_default().to_string()),
            node_updated_by_id: value.node_updated_by.as_ref().map(|user| user.id),
        }
    }
}

impl From<AuditUserPermission> for SerializedUserPermissions {
    fn from(value: AuditUserPermission) -> Self {
        Self {
            user_id: value.user_id,
            user_login: value.user_login,
            user_first_name: value.user_first_name,
            user_last_name: value.user_last_name,
            permissions: value.permissions,
        }
    }
}
