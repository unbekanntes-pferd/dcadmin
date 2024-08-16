use dco3::user::CustomerData;
use serde::Serialize;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SerializedCustomerInfo {
    pub space_limit: u64,
    pub space_used: u64,
    pub user_count: u64,
    pub user_limit: u64,
    pub cnt_internal_user: Option<u64>,
    pub cnt_guest_user: Option<u64>,
    pub encryption_enabled: bool,
}

impl From<CustomerData> for SerializedCustomerInfo {
    fn from(value: CustomerData) -> Self {
        SerializedCustomerInfo {
            space_limit: value.space_limit,
            space_used: value.space_used,
            user_count: value.accounts_used,
            user_limit: value.accounts_limit,
            cnt_internal_user: value.cnt_internal_user,
            cnt_guest_user: value.cnt_guest_user,
            encryption_enabled: value.customer_encryption_enabled,
        }
    }
}