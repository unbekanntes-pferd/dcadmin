import type { ListParams } from "$lib/models";
import { invoke } from "@tauri-apps/api/core";
import type { NodePermissions, NodePermissionsListEntry } from "./models";

export const getPermissions = async (params: ListParams) => {
    try {
        let permissionsList: NodePermissionsListEntry[] = await invoke('get_permissions', { params });
        return permissionsList.filter((entry) => entry.nodeCreatedById !== 0);
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

export enum PermissionsTemplate {
    RoomAdministator = 'Room Administrator',
    Edit = 'Edit',
    Read = 'Read',
    Custom = 'Custom',
    None = 'None'
}

export const displayPermissionsTemplate = (perms: NodePermissions): PermissionsTemplate => {
    if (!perms) {
        return PermissionsTemplate.None;
    }

    if (
        perms.manage &&
        perms.read &&
        perms.change &&
        perms.delete &&
        perms.create &&
        perms.manageDownloadShare &&
        perms.manageUploadShare &&
        perms.readRecycleBin &&
        perms.restoreRecycleBin &&
        perms.deleteRecycleBin
    ) {
        return PermissionsTemplate.RoomAdministator;
    }

    if (
        perms.read &&
        perms.change &&
        perms.delete &&
        perms.create &&
        perms.manageDownloadShare &&
        perms.manageUploadShare &&
        perms.readRecycleBin &&
        perms.restoreRecycleBin
    ) {
        return PermissionsTemplate.Edit;
    }

    if (perms.read && perms.manageDownloadShare) {
        return PermissionsTemplate.Read;
    }

    return PermissionsTemplate.Custom;
};