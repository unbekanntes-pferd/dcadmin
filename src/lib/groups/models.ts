import type { Range } from "$lib/models";
import type { RoleList } from "$lib/users/models";

export interface Group {
    id: number;
    name: string;
    createdAt: string;
    updatedAt?: string;
    createdById: number;
    updatedById: number;
    createdByName?: string;
    updatedByName?: string;
    createdByUserName?: string;
    updatedByUserName?: string;
    cntUsers?: number;
    expireAt?: number;
    groupRoles?: RoleList;
}

export interface GroupList {
    range: Range
    items: Group[]
}

export interface GroupUser {
    id: number;
    firstName?: string;
    lastName?: string;
    userName: string;
    email?: string;
}

export interface GroupUserList {
    range: Range
    items: GroupUser[]
}

export interface GroupInfo {
    id: number;
    name: string;
    cntUsers?: number;
}
