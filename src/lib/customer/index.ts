import { invoke } from "@tauri-apps/api";
import type { CustomerInfo } from "./models";

export const getCustomerInfo = async (): Promise<CustomerInfo> => {
    try {
        let customer_info: CustomerInfo = await invoke('get_customer_info');
        return customer_info;
    }
    catch (error) {
        console.error(error);
        throw error;
    }
}