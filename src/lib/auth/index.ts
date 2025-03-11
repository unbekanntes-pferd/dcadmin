import { invoke } from "@tauri-apps/api/core";
import type { UserAccount } from "./models";

export const initAuthCodeFlow = async (url: string): Promise<boolean> => {
    try {
         const isRefreshToken: boolean = await invoke('init_auth_code_flow', { url });
         return isRefreshToken; 
    }
    catch (error) {
        console.error(error);
        throw error;
    }
}

export const connect = async (isRefreshToken: boolean, authCode?: string): Promise<UserAccount> => {
    try {
        if (isRefreshToken) {
            let user_info: UserAccount = await invoke('connect');
            return user_info;
        }
        let user_info: UserAccount = await invoke('connect', { authCode });
        return user_info;
    }
    catch (error) {
        console.error(error);
        throw error;
    }
}