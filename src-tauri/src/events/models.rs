use chrono::{DateTime, Utc};
use std::hash::Hash;
use dco3::eventlog::{
    EventStatus, EventlogParams, LogEvent, LogEventList, LogOperation, LogOperationList,
};
use serde::{Deserialize, Serialize};

use crate::models::Range;

#[derive(PartialEq, Eq)]
pub struct EventsCacheKey {
    url: String,
    params: EventListParams
}

impl EventsCacheKey {
    pub fn new(url: String, params: EventListParams) -> Self {
        Self {
            url,
            params
        }
    }
}

impl From<&EventsCacheKey> for String {
    fn from(value: &EventsCacheKey) -> Self {
        format!("{}{}", value.url, value.params.to_string())
    }
}

impl Hash for EventsCacheKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let key: String = self.into();
        key.hash(state);
    }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SerializedEventList {
    pub range: Range,
    pub events: Vec<SerializedEvent>,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SerializedEvent {
    time: DateTime<Utc>,
    user_id: i64,
    message: String,
    user_name: Option<String>,
    status: Option<String>,
    operation_id: Option<i64>,
    operation_name: Option<String>,
    auth_parent_source: Option<String>,
    auth_parent_target: Option<String>,
    object_id1: Option<i64>,
    object_id2: Option<i64>,
    object_name1: Option<String>,
    object_name2: Option<String>,
    object_type1: Option<i64>,
    object_type2: Option<i64>,
    attribute1: Option<String>,
    attribute2: Option<String>,
    attribute3: Option<String>,
}

#[derive(Deserialize, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventListParams {
    pub offset: Option<u64>,
    pub limit: Option<u64>,
    pub user_id: Option<i64>,
    pub operation_type: Option<i64>,
    pub from_date: Option<String>,
    pub to_date: Option<String>,
    pub status: Option<u8>,
}

impl EventListParams {
    pub fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
    
}



impl TryFrom<EventListParams> for EventlogParams {
    type Error = String;
    fn try_from(value: EventListParams) -> Result<EventlogParams, String> {
        let date_start = value
            .from_date
            .as_ref()
            .map(|s| {
                chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S.%fZ")
                    .map(|dt| dt.and_utc())
                    .map_err(|e| {
            
                        e.to_string()
                    })
            })
            .transpose()?;

        let date_end = value
            .to_date
            .as_ref()
            .map(|s| {
                chrono::NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M:%S.%fZ")
                    .map(|dt| dt.and_utc())
                    .map_err(|e| e.to_string())
            })
            .transpose()?;

        let status = value
            .status
            .as_ref()
            .map(|status| match status {
                0 => Some(EventStatus::Success),
                2 => Some(EventStatus::Failure),
                _ => None,
            })
            .flatten();

        Ok(EventlogParams {
            offset: value.offset,
            limit: value.limit,
            user_id: value.user_id,
            operation_type: value.operation_type,
            date_start,
            date_end,
            status,
            user_client: None,
            sort: None,
        })
    }
}

impl From<LogEvent> for SerializedEvent {
    fn from(value: LogEvent) -> Self {
        SerializedEvent {
            time: value.time,
            user_id: value.user_id,
            message: value.message,
            user_name: value.user_name,
            status: value.status.map(|s| match s {
                EventStatus::Success => "success".to_string(),
                EventStatus::Failure => "failure".to_string(),
            }),
            operation_id: value.operation_id,
            operation_name: value.operation_name,
            auth_parent_source: value.auth_parent_source,
            auth_parent_target: value.auth_parent_target,
            object_id1: value.object_id1,
            object_id2: value.object_id2,
            object_name1: value.object_name1,
            object_name2: value.object_name2,
            object_type1: value.object_type1,
            object_type2: value.object_type2,
            attribute1: value.attribute1,
            attribute2: value.attribute2,
            attribute3: value.attribute3,
        }
    }
}

impl From<LogEventList> for SerializedEventList {
    fn from(value: LogEventList) -> Self {
        SerializedEventList {
            range: Range {
                offset: value.range.offset,
                limit: value.range.limit,
                total: value.range.total,
            },
            events: value.items.into_iter().map(|event| event.into()).collect(),
        }
    }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SerializedOperationTypes {
    pub operations: Vec<SerializedOperation>,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SerializedOperation {
    pub id: i64,
    pub name: String,
}

impl From<LogOperation> for SerializedOperation {
    fn from(value: LogOperation) -> Self {
        SerializedOperation {
            id: value.id,
            name: value.name,
        }
    }
}

impl From<LogOperationList> for SerializedOperationTypes {
    fn from(value: LogOperationList) -> Self {
        SerializedOperationTypes {
            operations: value
                .operation_list
                .into_iter()
                .map(|op| op.into())
                .collect(),
        }
    }
}
