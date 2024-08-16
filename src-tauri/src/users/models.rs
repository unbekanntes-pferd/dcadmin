use dco3::{
    user::{Role, RoleList},
    users::{UserItem, UserList},
};
use serde::Serialize;

use crate::models::Range;

const CONFIG_MANAGER: &str = "CONFIG_MANAGER";
const ROOM_MANAGER: &str = "ROOM_MANAGER";
const USER_MANAGER: &str = "USER_MANAGER";
const GROUP_MANAGER: &str = "GROUP_MANAGER";
const AUDITOR: &str = "LOG_AUDITOR";
const GUEST_USER: &str = "GUEST_USER";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SerializedUserList {
    pub range: Range,
    pub items: Vec<SerializedUserItem>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SerializedUserItem {
    pub id: u64,
    pub first_name: String,
    pub last_name: String,
    pub user_name: String,
    pub email: Option<String>,
    pub last_login: Option<String>,
    pub is_locked: bool,
    pub user_roles: Option<SerializedRoleList>, //TODO: Add remaining fields if needed
}

#[derive(Serialize)]
pub struct FlattenedUserItem {
    pub id: u64,
    pub first_name: String,
    pub last_name: String,
    pub user_name: String,
    pub email: Option<String>,
    pub last_login: Option<String>,
    pub is_locked: bool,
    pub is_config_manager: bool,
    pub is_room_manager: bool,
    pub is_user_manager: bool,
    pub is_group_manager: bool,
    pub is_auditor: bool,
    pub is_guest_user: bool,
}

impl From<SerializedUserItem> for FlattenedUserItem {
    fn from(value: SerializedUserItem) -> Self {
        FlattenedUserItem {
            id: value.id,
            first_name: value.first_name,
            last_name: value.last_name,
            user_name: value.user_name,
            email: value.email,
            last_login: value.last_login,
            is_locked: value.is_locked,
            is_config_manager: value
                .user_roles
                .as_ref()
                .map(|roles| roles.items.iter().any(|role| role.name == CONFIG_MANAGER))
                .unwrap_or(false),
            is_room_manager: value
                .user_roles
                .as_ref()
                .map(|roles| roles.items.iter().any(|role| role.name == ROOM_MANAGER))
                .unwrap_or(false),
            is_user_manager: value
                .user_roles
                .as_ref()
                .map(|roles| roles.items.iter().any(|role| role.name == USER_MANAGER))
                .unwrap_or(false),
            is_group_manager: value
                .user_roles
                .as_ref()
                .map(|roles| roles.items.iter().any(|role| role.name == GROUP_MANAGER))
                .unwrap_or(false),
            is_auditor: value
                .user_roles
                .as_ref()
                .map(|roles| roles.items.iter().any(|role| role.name == AUDITOR))
                .unwrap_or(false),
            is_guest_user: value
                .user_roles
                .as_ref()
                .map(|roles| roles.items.iter().any(|role| role.name == GUEST_USER))
                .unwrap_or(false),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SerializedRoleList {
    pub items: Vec<SerializedRoleItem>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SerializedRoleItem {
    pub id: u64,
    pub name: String,
    pub description: String,
}

impl From<RoleList> for SerializedRoleList {
    fn from(value: RoleList) -> Self {
        SerializedRoleList {
            items: value.items.into_iter().map(|role| role.into()).collect(),
        }
    }
}

impl From<Role> for SerializedRoleItem {
    fn from(value: Role) -> Self {
        SerializedRoleItem {
            id: value.id,
            name: value.name,
            description: value.description,
        }
    }
}

impl From<UserItem> for SerializedUserItem {
    fn from(value: UserItem) -> Self {
        SerializedUserItem {
            id: value.id,
            first_name: value.first_name,
            last_name: value.last_name,
            user_name: value.user_name,
            email: value.email,
            last_login: value.last_login_success_at,
            is_locked: value.is_locked,
            user_roles: value.user_roles.map(|roles| roles.into()),
        }
    }
}

impl From<UserList> for SerializedUserList {
    fn from(value: UserList) -> Self {
        SerializedUserList {
            range: Range {
                offset: value.range.offset,
                limit: value.range.limit,
                total: value.range.total,
            },
            items: value.items.into_iter().map(|user| user.into()).collect(),
        }
    }
}
