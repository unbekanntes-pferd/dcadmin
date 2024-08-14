export interface Range {
    offset: number;
    limit: number;
    total: number;
}

export interface ListParams {
    offset?: number;
    limit?: number;
    filter?: string;
    sort?: string;
}

export enum ToastType {
    Success = 'variant-filled-success',
    Error = 'variant-filled-error',
    Warning = 'variant-filled-warning',
    Info = 'variant-filled-primary'
}