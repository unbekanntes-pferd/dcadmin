import { goto } from "$app/navigation";
import type { ToastType } from "$lib/models";
import type { ToastSettings } from "@skeletonlabs/skeleton";

export const toReadableSize = (size: number): string => {
    const units = ["B", "KB", "MB", "GB", "TB", "PB"];
    if (size === 0) {
        return "0 B";
    }
    const exp = Math.min(
        Math.floor(Math.log(size) / Math.log(1024)),
        units.length - 1
    );
    
    const divisor = Math.pow(1024, exp);
    const sizeWhole = Math.floor(size / divisor);
    const sizeFrac = Math.floor(((size % divisor) * 10 + divisor / 2) / divisor);
    
    if (exp === 0) {
        return `${sizeWhole} ${units[exp]}`;
    } else if (sizeFrac === 0) {
        return `${sizeWhole} ${units[exp]}`;
    } else {
        return `${sizeWhole}.${sizeFrac} ${units[exp]}`;
    }
}

export const formatUTCDateTime = (dateString: string): string => {
    const date = new Date(dateString);
    return date.toLocaleString('en-US', {
        year: 'numeric',
        month: 'long',
        day: 'numeric',
        hour: '2-digit',
        minute: '2-digit',
        second: '2-digit',
        timeZone: 'UTC',
        timeZoneName: 'short'
    });
};

export const createToastSettings = (message: string, toastType: ToastType, timeout?: number): ToastSettings => {
    return {
        message,
        background: toastType,
        timeout: timeout ? timeout : 3000,
        autohide: true
    }
}

export const handleNodeNavigation = async (id: number, permissions?: boolean) => {
    console.log(permissions, id);
    if (permissions) {
        await goto(`/nodes/${id}/permissions`);
        return;
    }
    await goto(`/nodes/${id}`);
};