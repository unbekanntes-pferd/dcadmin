import { invoke } from "@tauri-apps/api/core";
import type { LastAdminUserRoomList, UserList } from "./models";
import type { ListParams } from "$lib/models";

export const getUsers = async (params: ListParams): Promise<UserList> => {
    try {
        let users: UserList = await invoke('get_users', { params });

        return users;
    }
    catch (error) {
        console.error(error);
        throw error;
    }
}

export const downloadUsers = async (params: ListParams, path: string): Promise<void> => {
    try {
        await invoke('export_users', { params, path });
    }
    catch (error) {
        console.error(error);
        throw error;
    }
}

export const getLastAdminUserRooms = async (userId: number): Promise<LastAdminUserRoomList> => {
    try {
        let rooms: LastAdminUserRoomList = await invoke('get_user_last_admin_rooms', { userId });  
        
        return rooms;
    }
    catch (error) {
        console.error(error);
        throw error;
    }
}
