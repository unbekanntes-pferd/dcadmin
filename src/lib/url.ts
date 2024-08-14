import { invoke } from "@tauri-apps/api";

export const validateUrl = async (url: string): Promise<boolean> => {
    const urlRegex = /^(https?:\/\/)([\w.-]+)(\.[\w.-]+)+([\/\w\.-]*)*\/?$/;
    return urlRegex.test(url) && await checkDracoonUrl(url);
};

export const addHttps = (url: string): string => {
    if (url.startsWith("http://")) {
        return url.replace("http://", "https://");
    }

    if (url.startsWith("https://")) {
        return url;
    }

    return `https://${url}`;
};

const checkDracoonUrl = async (url: string): Promise<boolean> => {
    return invoke('validate_dracoon_url', { url });
}