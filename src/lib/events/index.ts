import { invoke } from "@tauri-apps/api";
import type { EventList, EventParams, OperationTypeList } from "./models";

export const getEvents = async (params: EventParams): Promise<EventList> => {
    try {
        let events: EventList = await invoke('get_events', { params });
        return events;
    }
    catch (error) {
        console.error(error);
        throw error;
    }
}

export const downloadEvents = async (path: string, params: EventParams): Promise<void> => {
    try {
        await invoke('export_events', { path, params });
    }
    catch (error) {
        console.error(error);
        throw error;
    }
}

export const getOperationTypes = async (): Promise<OperationTypeList> => {
    try {
        let operationTypes: OperationTypeList = await invoke('get_operation_types');
        return operationTypes;
    }
    catch (error) {
        console.error(error);
        throw error;
    }
}