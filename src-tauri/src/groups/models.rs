use dco3::groups::{Group, GroupList, GroupUser, GroupUserList};
use serde::Serialize;

use crate::{models::Range, users::SerializedRoleList, ROLE_AUDITOR, ROLE_CONFIG_MANAGER, ROLE_GROUP_MANAGER, ROLE_ROOM_MANAGER, ROLE_USER_MANAGER};

#[derive(Serialize)]
pub struct SerializedGroupInfo {
    pub id: u64,
    pub name: String,
    pub cnt_users: Option<u64>,
}

impl From<Group> for SerializedGroupInfo {
    fn from(value: Group) -> Self {
        SerializedGroupInfo {
            id: value.id,
            name: value.name,
            cnt_users: value.cnt_users,
        }
    }
}

#[derive(Serialize)]
pub struct SerializedGroupList {
    pub items: Vec<SerializedGroup>,
    pub range: Range,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SerializedGroup {
    pub id: u64,
    pub name: String,
    pub created_at: String,
    pub created_by_id: i64,
    pub created_by_name: Option<String>,
    pub created_by_user_name: Option<String>,
    pub updated_at: Option<String>,
    pub updated_by_id: Option<i64>,
    pub updated_by_name: Option<String>,
    pub updated_by_user_name: Option<String>,
    pub cnt_users: Option<u64>,
    pub expire_at: Option<String>,
    pub group_roles: Option<SerializedRoleList>,
}

#[derive(Serialize)]
pub struct FlattenedSerializedGroup {
    pub id: u64,
    pub name: String,
    pub created_at: String,
    pub created_by_id: i64,
    pub created_by_name: Option<String>,
    pub created_by_user_name: Option<String>,
    pub updated_at: Option<String>,
    pub updated_by_id: Option<i64>,
    pub updated_by_name: Option<String>,
    pub updated_by_user_name: Option<String>,
    pub cnt_users: Option<u64>,
    pub expire_at: Option<String>,
    pub is_config_manager: bool,
    pub is_room_manager: bool,
    pub is_user_manager: bool,
    pub is_group_manager: bool,
    pub is_auditor: bool,
}

impl From<SerializedGroup> for FlattenedSerializedGroup {
    fn from(value: SerializedGroup) -> Self {
        FlattenedSerializedGroup {
            id: value.id,
            name: value.name,
            created_at: value.created_at,
            created_by_id: value.created_by_id,
            created_by_name: value.created_by_name,
            created_by_user_name: value.created_by_user_name,
            updated_at: value.updated_at,
            updated_by_id: value.updated_by_id,
            updated_by_name: value.updated_by_name,
            updated_by_user_name: value.updated_by_user_name,
            cnt_users: value.cnt_users,
            expire_at: value.expire_at,
            is_config_manager: value
                .group_roles
                .as_ref()
                .map(|roles| roles.items.iter().any(|role| role.name == ROLE_CONFIG_MANAGER))
                .unwrap_or_default(),
            is_room_manager: value
                .group_roles
                .as_ref()
                .map(|roles| roles.items.iter().any(|role| role.name == ROLE_ROOM_MANAGER))
                .unwrap_or_default(),
            is_user_manager: value
                .group_roles
                .as_ref()
                .map(|roles| roles.items.iter().any(|role| role.name == ROLE_USER_MANAGER))
                .unwrap_or_default(),
            is_group_manager: value
                .group_roles
                .as_ref()
                .map(|roles| roles.items.iter().any(|role| role.name == ROLE_GROUP_MANAGER))
                .unwrap_or_default(),
            is_auditor: value
                .group_roles
                .as_ref()
                .map(|roles| roles.items.iter().any(|role| role.name == ROLE_AUDITOR))
                .unwrap_or_default(),
        }
    }
}

impl From<Group> for SerializedGroup {
    fn from(value: Group) -> Self {
        SerializedGroup {
            id: value.id,
            name: value.name,
            created_at: value
                .created_at
                .to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            created_by_id: value.created_by.id,
            created_by_name: value.created_by.first_name.map(|first_name| {
                format!(
                    "{} {}",
                    first_name,
                    value.created_by.last_name.unwrap_or_default()
                )
            }),
            created_by_user_name: value.created_by.user_name,
            updated_at: value
                .updated_at
                .map(|date| date.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)),
            updated_by_id: value.updated_by.as_ref().map(|user| user.id),
            updated_by_name: value.updated_by.as_ref().map(|user| {
                format!(
                    "{} {}",
                    user.first_name.clone().unwrap_or_default(),
                    user.last_name.clone().unwrap_or_default()
                )
            }),
            updated_by_user_name: value
                .updated_by
                .as_ref()
                .map(|user| user.user_name.clone().unwrap_or_default()),
            cnt_users: value.cnt_users,
            expire_at: value
                .expire_at
                .map(|date| date.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)),
            group_roles: value.group_roles.map(|roles| roles.into()),
        }
    }
}

impl From<GroupList> for SerializedGroupList {
    fn from(value: GroupList) -> Self {
        SerializedGroupList {
            items: value.items.into_iter().map(SerializedGroup::from).collect(),
            range: value.range.into(),
        }
    }
}


#[derive(Serialize)]
pub struct SerializedGroupUserList {
    pub items: Vec<SerializedGroupUser>,
    pub range: Range,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SerializedGroupUser {
    pub id: i64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub user_name: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SerializedGroupUserWithGroupInfo {
    pub id: i64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub user_name: Option<String>,
    pub group_id: u64,
    pub group_name: String,
}

impl From<(Group, GroupUser)> for SerializedGroupUserWithGroupInfo {
    fn from((group, user): (Group, GroupUser)) -> Self {
        SerializedGroupUserWithGroupInfo {
            id: user.user_info.id,
            first_name: user.user_info.first_name,
            last_name: user.user_info.last_name,
            email: user.user_info.email,
            user_name: user.user_info.user_name,
            group_id: group.id,
            group_name: group.name,
        }
    }
}

impl From<GroupUser> for SerializedGroupUser {
    fn from(value: GroupUser) -> Self {
        SerializedGroupUser {
            id: value.user_info.id,
            first_name: value.user_info.first_name,
            last_name: value.user_info.last_name,
            email: value.user_info.email,
            user_name: value.user_info.user_name,
        }
    }
}

impl From<GroupUserList> for SerializedGroupUserList {
    fn from(value: GroupUserList) -> Self {
        SerializedGroupUserList {
            items: value.items.into_iter().map(SerializedGroupUser::from).collect(),
            range: value.range.into(),
        }
    }
}