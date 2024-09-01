import type { ListParams } from "$lib/models";
import { invoke } from "@tauri-apps/api";
import type { GroupInfo, GroupList, GroupUserList } from "./models";

export const getGroup = async (groupId: number): Promise<GroupInfo> => {
    try {
        let group: GroupInfo = await invoke('get_group', { groupId });
        return group;
    }
    catch (error) {
        console.error(error);
        throw error;
    }
}

export const getGroups = async (params: ListParams): Promise<GroupList> => {
    try {
        let groups: GroupList = await invoke('get_groups', { params });

        console.log(groups);

        return groups;
    }
    catch (error) {
        console.error(error);
        throw error;
    }
}

export const downloadGroups = async (params: ListParams, path: string): Promise<void> => {
    try {
        await invoke('export_groups', { params, path });
    }
    catch (error) {
        console.error(error);
        throw error;
    }
}

export const getGroupUsers = async (groupId: number, params: ListParams): Promise<GroupUserList> => {
    try {
        let users: GroupUserList = await invoke('get_group_users', { groupId, params });
        return users;
    }
    catch (error) {
        console.error(error);
        throw error;
    }
}

export const downloadGroupUsers = async (groupId: number, params: ListParams, path: string): Promise<void> => {
    try {
        await invoke('export_group_users', { groupId, params, path });
    }
    catch (error) {
        console.error(error);
        throw error;
    }
}

export const downloadAllGroupUsers = async (path: string): Promise<void> => {
    try {
        await invoke('export_all_group_users', { path });
    }
    catch (error) {
        console.error(error);
        throw error;
    }
}