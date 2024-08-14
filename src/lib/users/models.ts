import type { Range } from "$lib/models";

export interface UserItem {
    id: number;
    firstName: string;
    lastName: string;
    userName: string;
    email?: string;
    isLocked: boolean;
    lastLogin: string;
}

export interface UserList {
    range: Range
    items: UserItem[]
}
