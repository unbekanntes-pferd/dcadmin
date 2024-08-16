import type { Range } from "$lib/models";

export interface UserItem {
    id: number;
    firstName: string;
    lastName: string;
    userName: string;
    email?: string;
    isLocked: boolean;
    lastLogin: string;
    userRoles?: RoleList
}

export interface UserList {
    range: Range
    items: UserItem[]
}

export interface RoleList {
    items: RoleItem[]
}

export interface RoleItem {
    id: number;
    roleName: string;
    description: string;
}

export const roleList = [
    { label: 'Config Manager', value: 'CONFIG_MANAGER' },
    { label: 'Room Manager', value: 'ROOM_MANAGER' },
    { label: 'User Manager', value: 'USER_MANAGER' },
    { label: 'Group Manager', value: 'GROUP_MANAGER' },
    { label: 'Auditor', value: 'AUDITOR' },
    { label: 'User', value: 'USER' },
    { label: 'Guest user', value: 'GUEST_USER' }
];