export interface NodePermissionsList {
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