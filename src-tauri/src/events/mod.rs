mod models;
use std::{sync::Arc, time::Instant};

use crate::AppState;
use dco3::{eventlog::EventlogParams, Eventlog};
use models::{EventListParams, SerializedEvent};
pub use models::{EventsCacheKey, SerializedEventList, SerializedOperationTypes};
use tauri::State;

#[tauri::command]
pub async fn get_events(
    params: EventListParams,
    state: State<'_, AppState>,
) -> Result<SerializedEventList, String> {
    let now = Instant::now();
    let client = state.get_client().await?;

    let key = EventsCacheKey::new(client.get_base_url().to_string(), params.clone());

    if let Some(events) = state.get_events_cache().get(&key).await {
        let elapsed = now.elapsed().as_millis();
        tracing::info!("Fetched cached events in {elapsed} ms");
        return Ok((*events).clone());
    }

    let events = client
        .eventlog()
        .get_events(params.clone().try_into()?)
        .await
        .map_err(|e| e.to_string())?;

    let serialized_events: SerializedEventList = events.into();

    let serialized_events = Arc::new(serialized_events);

    state
        .get_events_cache()
        .insert(key, serialized_events.clone())
        .await;

    let elapsed = now.elapsed().as_millis();
    tracing::info!("Fetched events in {elapsed} ms");
    Ok((*serialized_events).clone())
}

#[tauri::command]
pub async fn export_events(
    params: EventListParams,
    path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let now = Instant::now();
    let client = state.get_client().await?;

    let mut events = client
        .eventlog()
        .get_events(params.clone().try_into()?)
        .await
        .map_err(|e| e.to_string())?;

    for offset in (500..events.range.total).step_by(500) {
        tracing::debug!("Fetching events with offset {offset}");
        let params = EventlogParams {
            offset: Some(offset),
            ..params.clone().try_into()?
        };

        let new_events = client
            .eventlog()
            .get_events(params)
            .await
            .map_err(|e| e.to_string())?;

        events.items.extend(new_events.items);
    }

    let elapsed_all_events = now.elapsed().as_millis();
    tracing::info!("Fetched all events in {elapsed_all_events} ms");

    let serializable_events: Vec<SerializedEvent> = events
        .items
        .into_iter()
        .map(|e| e.into())
        .collect::<Vec<_>>();

    let mut csv_writer = csv::Writer::from_path(path).map_err(|e| e.to_string())?;

    for event in serializable_events {
        csv_writer.serialize(event).map_err(|e| e.to_string())?;
    }

    csv_writer.flush().map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn get_operation_types(
    state: State<'_, AppState>,
) -> Result<SerializedOperationTypes, String> {
    let noe = Instant::now();
    let client = state.get_client().await?;

    let key = client.get_base_url().to_string();

    if let Some(operation_types) = state.get_operations_cache().get(&key).await {
        let elapsed = noe.elapsed().as_millis();
        tracing::info!("Fetched cached operation types in {elapsed} ms");
        return Ok((*operation_types).clone());
    }

    let operation_types: SerializedOperationTypes = client
        .eventlog()
        .get_event_operations()
        .await
        .map_err(|e| e.to_string())?.into();

    let operation_types = Arc::new(operation_types);

    state
        .get_operations_cache()
        .insert(key, operation_types.clone())
        .await;

    let elapsed = noe.elapsed().as_millis();
    tracing::info!("Fetched operation types in {elapsed} ms");
    Ok((*operation_types).clone())
}
