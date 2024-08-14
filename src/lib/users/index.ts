import { invoke } from "@tauri-apps/api";
import type { UserList } from "./models";
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
