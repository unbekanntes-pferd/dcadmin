export interface CustomerInfo {
    spaceLimit: number;
    spaceUsed: number;
    userCount: number;
    userLimit: number;
    cntInternalUser?: number;
    cntGuestUser?: number;
    encryptionEnabled: boolean;
}