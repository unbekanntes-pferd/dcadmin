import type { ListParams } from "$lib/models";
import { invoke } from "@tauri-apps/api";
import type { NodePermissionsListEntry } from "./models";

export const getPermissions = async (params: ListParams) => {
    try {
        let permissionsList: NodePermissionsListEntry[] = await invoke('get_permissions', { params });
        return permissionsList;
    }
    catch (error) {
        console.error(error);
        throw error;
    }
}

export const downloadUserPermissions = async (path: string, params: ListParams): Promise<void> => {
    try {
        await invoke('export_user_permissions', { path, params });
    }
    catch (error) {
        console.error(error);
        throw error;
    }
}

export const downloadAllUserPermissions = async (path: string): Promise<void> => {
    try {
        await invoke('export_all_user_permissions', { path });
    }
    catch (error) {
        console.error(error);
        throw error;
    }
}
