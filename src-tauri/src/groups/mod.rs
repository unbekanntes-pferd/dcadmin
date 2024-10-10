use std::time::Instant;

use crate::{config::log_dracoon_error, models::ListParams, AppState};
use dco3::{
    auth::Connected,
    groups::{GroupList, GroupUserList},
    Dracoon, Groups,
};
use models::{
    FlattenedSerializedGroup, SerializedGroupInfo, SerializedGroupList, SerializedGroupUserList,
    SerializedGroupUserWithGroupInfo,
};
use tauri::State;
mod models;

#[tauri::command]
pub async fn get_group(
    group_id: u64,
    state: State<'_, AppState>,
) -> Result<SerializedGroupInfo, String> {
    let now = Instant::now();
    let client = state.get_client().await?;

    let group = client.groups().get_group(group_id).await.map_err(|e| {
        log_dracoon_error(&e, Some("Error fetching group"));
        e.to_string()
    })?;

    let elapsed = now.elapsed().as_millis();
    tracing::info!("Fetched group {group_id} in {elapsed} ms");

    Ok(group.into())
}

#[tauri::command]
pub async fn get_groups(
    params: ListParams,
    state: State<'_, AppState>,
) -> Result<SerializedGroupList, String> {
    let now = Instant::now();
    let client = state.get_client().await?;
    let params = params.try_into()?;

    let groups = client
        .groups()
        .get_groups(Some(params))
        .await
        .map_err(|e| {
            log_dracoon_error(&e, Some("Error fetching groups"));
            e.to_string()
        })?;

    let group_count = groups.items.len();

    let elapsed = now.elapsed().as_millis();
    tracing::info!("Fetched {group_count} groups in {elapsed} ms");
    Ok(groups.into())
}

#[tauri::command]
pub async fn export_groups(
    params: ListParams,
    path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let now = Instant::now();

    let client = state.get_client().await?;

    let mut groups = client
        .groups()
        .get_groups(Some(params.clone().try_into()?))
        .await
        .map_err(|e| {
            log_dracoon_error(&e, Some("Error fetching groups"));
            e.to_string()
        })?;

    let elapsed_first_500_groups = now.elapsed().as_millis();
    let group_count = groups.items.len();

    tracing::info!("Fetched first {group_count} groups in {elapsed_first_500_groups} ms");

    for offset in (500..groups.range.total).step_by(500) {
        tracing::debug!("Fetching groups with offset {offset}");
        let params = ListParams {
            offset: Some(offset),
            ..params.clone()
        };

        let new_groups = client
            .groups()
            .get_groups(Some(params.try_into()?))
            .await
            .map_err(|e| {
                log_dracoon_error(&e, Some("Error fetching groups"));
                e.to_string()
            })?;

        groups.items.extend(new_groups.items);
    }

    let elapsed = now.elapsed().as_millis();
    tracing::info!("Fetched all groups in {elapsed} ms");

    let serialized_groups: SerializedGroupList = groups.into();
    let flattened_groups: Vec<FlattenedSerializedGroup> = serialized_groups
        .items
        .into_iter()
        .map(|group| group.into())
        .collect();

    let mut csv_writer = csv::Writer::from_path(path).map_err(|e| {
        tracing::error!("Error creating CSV writer: {e}");
        e.to_string()
    })?;

    for group in flattened_groups {
        csv_writer.serialize(group).map_err(|e| {
            tracing::error!("Error serializing group: {e}");
            e.to_string()
        })?;
    }

    csv_writer.flush().map_err(|e| {
        tracing::error!("Error flushing CSV writer: {e}");
        e.to_string()
    })?;

    let elapsed_csv = now.elapsed().as_millis();
    tracing::info!("Exported all groups to CSV in {elapsed_csv} ms");

    Ok(())
}

#[tauri::command]
pub async fn get_group_users(
    group_id: u64,
    params: ListParams,
    state: State<'_, AppState>,
) -> Result<SerializedGroupUserList, String> {
    let now = Instant::now();
    let client = state.get_client().await?;
    let params = params.try_into()?;

    dbg!(&params);

    let group_users = client
        .groups()
        .get_group_users(group_id, Some(params))
        .await
        .map_err(|e| {
            log_dracoon_error(&e, Some("Error fetching group users"));
            e.to_string()
        })?;

    let elapsed = now.elapsed().as_millis();
    let user_count = group_users.items.len();
    tracing::info!("Fetched {user_count} group users in {elapsed} ms");

    Ok(group_users.into())
}

#[tauri::command]
pub async fn export_group_users(
    path: String,
    group_id: u64,
    params: ListParams,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let now = Instant::now();

    let client = state.get_client().await?;

    let group_users = get_all_group_users(&client, group_id, params.clone()).await?;

    let elapsed = now.elapsed().as_millis();

    tracing::info!("Fetched all group users in {elapsed} ms");

    let serialized_group_users: SerializedGroupUserList = group_users.into();

    let mut csv_writer = csv::Writer::from_path(path).map_err(|e| {
        tracing::error!("Error creating CSV writer: {e}");
        e.to_string()
    })?;

    for group_user in serialized_group_users.items {
        csv_writer.serialize(group_user).map_err(|e| {
            tracing::error!("Error serializing group user: {e}");
            e.to_string()
        })?;
    }

    csv_writer.flush().map_err(|e| {
        tracing::error!("Error flushing CSV writer: {e}");
        e.to_string()
    })?;

    let elapsed_csv = now.elapsed().as_millis();
    tracing::info!("Exported all group users for group {group_id} to CSV in {elapsed_csv} ms");

    Ok(())
}

#[tauri::command]
pub async fn export_all_group_users(
    path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let now = Instant::now();
    let client = state.get_client().await?;

    let groups = get_all_groups(&client).await?;
    let elapsed_groups = now.elapsed().as_millis();
    let group_count = groups.items.len();
    tracing::info!("Fetched {group_count} groups in {elapsed_groups} ms");

    let mut all_group_users = Vec::new();

    for group in groups {
        let group_users = get_all_group_users(&client, group.id, ListParams::default()).await?;
        let serialized_group_users: Vec<SerializedGroupUserWithGroupInfo> = group_users
            .into_iter()
            .map(|user| (group.clone(), user).into())
            .collect();

        all_group_users.extend(serialized_group_users);
    }

    let mut csv_writer = csv::Writer::from_path(path.clone()).map_err(|e| {
        tracing::error!("Error creating CSV writer: {e}");
        e.to_string()
    })?;

    for group_user in all_group_users {
        csv_writer.serialize(group_user).map_err(|e| {
            tracing::error!("Error serializing group user: {e}");
            e.to_string()
        })?;
    }

    csv_writer.flush().map_err(|e| {
        tracing::error!("Error flushing CSV writer: {e}");
        e.to_string()
    })?;

    Ok(())
}

async fn get_all_groups(client: &Dracoon<Connected>) -> Result<GroupList, String> {
    let mut groups = client.groups().get_groups(None).await.map_err(|e| {
        log_dracoon_error(&e, Some("Error fetching groups"));
        e.to_string()
    })?;

    if groups.range.total > 500 {
        for offset in (500..groups.range.total).step_by(500) {
            tracing::debug!("Fetching groups with offset {offset}");
            let params = ListParams {
                offset: Some(offset),
                ..Default::default()
            };

            let new_groups = client
                .groups()
                .get_groups(Some(params.try_into()?))
                .await
                .map_err(|e| {
                    log_dracoon_error(&e, Some("Error fetching groups"));
                    e.to_string()
                })?;

            groups.items.extend(new_groups.items);
        }
    }

    Ok(groups)
}

async fn get_all_group_users(
    client: &Dracoon<Connected>,
    group_id: u64,
    params: ListParams,
) -> Result<GroupUserList, String> {
    let mut group_users = client
        .groups()
        .get_group_users(group_id, Some(params.clone().try_into()?))
        .await
        .map_err(|e| {
            log_dracoon_error(&e, Some("Error fetching group users"));
            e.to_string()
        })?;

    if group_users.range.total > 500 {
        for offset in (500..group_users.range.total).step_by(500) {
            tracing::debug!("Fetching group users with offset {offset}");
            let params = ListParams {
                offset: Some(offset),
                ..params.clone()
            };

            let new_group_users = client
                .groups()
                .get_group_users(group_id, Some(params.try_into()?))
                .await
                .map_err(|e| {
                    log_dracoon_error(&e, Some("Error fetching group users"));
                    e.to_string()
                })?;

            group_users.items.extend(new_group_users.items);
        }
    }

    Ok(group_users)
}
