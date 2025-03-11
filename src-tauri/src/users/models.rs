use dco3::{
    roles::{Role, RoleList},
    users::{LastAdminUserRoom, LastAdminUserRoomList, UserItem, UserList},
};
use serde::Serialize;

use crate::{
    models::Range, ROLE_AUDITOR, ROLE_CONFIG_MANAGER, ROLE_GROUP_MANAGER, ROLE_GUEST_USER,
    ROLE_ROOM_MANAGER, ROLE_USER_MANAGER,
};

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
                .map(|roles| {
                    roles
                        .items
                        .iter()
                        .any(|role| role.name == ROLE_CONFIG_MANAGER)
                })
                .unwrap_or(false),
            is_room_manager: value
                .user_roles
                .as_ref()
                .map(|roles| {
                    roles
                        .items
                        .iter()
                        .any(|role| role.name == ROLE_ROOM_MANAGER)
                })
                .unwrap_or(false),
            is_user_manager: value
                .user_roles
                .as_ref()
                .map(|roles| {
                    roles
                        .items
                        .iter()
                        .any(|role| role.name == ROLE_USER_MANAGER)
                })
                .unwrap_or(false),
            is_group_manager: value
                .user_roles
                .as_ref()
                .map(|roles| {
                    roles
                        .items
                        .iter()
                        .any(|role| role.name == ROLE_GROUP_MANAGER)
                })
                .unwrap_or(false),
            is_auditor: value
                .user_roles
                .as_ref()
                .map(|roles| roles.items.iter().any(|role| role.name == ROLE_AUDITOR))
                .unwrap_or(false),
            is_guest_user: value
                .user_roles
                .as_ref()
                .map(|roles| roles.items.iter().any(|role| role.name == ROLE_GUEST_USER))
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


#[derive(Serialize, Clone)]
pub struct SerializedLastAdminUserRoomList {
    pub items: Vec<SerializedLastAdminUserRoom>,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SerializedLastAdminUserRoom {
    pub id: u64,
    pub name: String,
    pub parent_path: String,
    pub last_admin_in_group: bool,
    pub parent_id: Option<u64>,
    pub last_admin_in_group_id: Option<u64>,
}

impl From<LastAdminUserRoom> for SerializedLastAdminUserRoom {
    fn from(value: LastAdminUserRoom) -> Self {
        SerializedLastAdminUserRoom {
            id: value.id,
            name: value.name,
            parent_path: value.parent_path,
            last_admin_in_group: value.last_admin_in_group,
            parent_id: value.parent_id,
            last_admin_in_group_id: value.last_admin_in_group_id,
        }
    }
}

impl From<LastAdminUserRoomList> for SerializedLastAdminUserRoomList {
    fn from(value: LastAdminUserRoomList) -> Self {
        SerializedLastAdminUserRoomList {
            items: value.items.into_iter().map(|room| room.into()).collect(),
        }
    }
}

