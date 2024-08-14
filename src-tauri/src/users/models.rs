use dco3::users::{UserItem, UserList};
use serde::Serialize;

use crate::models::Range;

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
    //TODO: Add roles (!)
    //TODO: Add remaining fields if needed
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
