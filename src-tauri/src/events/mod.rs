mod models;
use crate::AppState;
use dco3::{eventlog::EventlogParams, Eventlog};
use models::{EventListParams, SerializedEvent, SerializedEventList, SerializedOperationTypes};
use tauri::State;

#[tauri::command]
pub async fn get_events(
    params: EventListParams,
    state: State<'_, AppState>,
) -> Result<SerializedEventList, String> {
    let client = state.get_client().await?;

    Ok(client
        .eventlog
        .get_events(params.try_into()?)
        .await
        .map_err(|e| e.to_string())?
        .into())
}

#[tauri::command]
pub async fn export_events(
    params: EventListParams,
    path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let client = state.get_client().await?;

    let mut events = client
        .eventlog
        .get_events(params.clone().try_into()?)
        .await
        .map_err(|e| e.to_string())?;

    for offset in (500..events.range.total).step_by(500) {
        let params = EventlogParams {
            offset: Some(offset),
            ..params.clone().try_into()?
        };

        let new_events = client
            .eventlog
            .get_events(params)
            .await
            .map_err(|e| e.to_string())?;

        events.items.extend(new_events.items);
    }

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
    let client = state.get_client().await?;

    Ok(client
        .eventlog
        .get_event_operations()
        .await
        .map_err(|e| e.to_string())?
        .into())
}
