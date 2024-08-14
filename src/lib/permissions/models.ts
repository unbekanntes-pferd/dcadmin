export interface NodePermissionsListEntry {
    nodeId: number;
    nodeName: string;
    nodeParentPath: string;
    nodeCntChildren: number;
    userPermissions: UserPermissions[];
    nodeParentId?: number;
    nodeSize?: number;
    nodeRecycleBinRetentionPeriod?: number;
    nodeQuota?: number;
    nodeIsEncrypted?: boolean;
    nodeHasActivitiesLog?: boolean;
    nodeCreatedAt?: string;
    nodeUpdatedAt?: string;
    nodeCreatedBy?: string;
    nodeUpdatedBy?: string;
    nodeCreatedById?: number;
    nodeUpdatedById?: number;
}

export interface UserPermissions {
    userId: number;
    userLogin: string;
    userFirstName: string;
    userLastName: string;
    permissions: NodePermissions;
}

export interface NodePermissions {
    manage: boolean;
    read: boolean;
    create: boolean;
    change: boolean;
    delete: boolean;
    manageDownloadShare: boolean;
    manageUploadShare: boolean;
    readRecycleBin: boolean;
    restoreRecycleBin: boolean;
    deleteRecycleBin: boolean;
}

export interface NodeInfo {
    id: number;
    name: string;
    parentPath: string;
    cntChildren: number;
    parentId?: number;
    size?: number;
    recycleBinRetentionPeriod?: number;
    quota?: number;
    isEncrypted?: boolean;
    hasActivitiesLog?: boolean;
    createdAt?: string;
    updatedAt?: string;
    createdBy?: string;
    updatedBy?: string;
    createdById?: number;
    updatedById?: number;
    cntPermissions?: number;
}

export const intoNodeInfo = (entry: NodePermissionsListEntry): NodeInfo => {
    return {
        id: entry.nodeId,
        name: entry.nodeName,
        parentPath: entry.nodeParentPath,
        cntChildren: entry.nodeCntChildren,
        parentId: entry.nodeParentId,
        size: entry.nodeSize,
        recycleBinRetentionPeriod: entry.nodeRecycleBinRetentionPeriod,
        quota: entry.nodeQuota,
        isEncrypted: entry.nodeIsEncrypted,
        hasActivitiesLog: entry.nodeHasActivitiesLog,
        createdAt: entry.nodeCreatedAt,
        updatedAt: entry.nodeUpdatedAt,
        createdBy: entry.nodeCreatedBy,
        updatedBy: entry.nodeUpdatedBy,
        createdById: entry.nodeCreatedById,
        updatedById: entry.nodeUpdatedById,
        cntPermissions: entry.userPermissions ? entry.userPermissions.length : 0
    }
}