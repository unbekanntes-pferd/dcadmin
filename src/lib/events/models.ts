import { type Range } from '../models';

export interface EventList {
    range: Range;
    events: Event[];
}

export interface Event {
    time: string;
    userId: number;
    message: string;
    userName?: string;
    status?: string;
    operationId?: number;
    operationName?: string;
    authParentSource?: string;
    authParentTarget?: string;
    objectId1?: number;
    objectName1?: string;
    objectType1?: number;
    objectId2?: number;
    objectName2?: string;
    objectType2?: number;
    attribute1?: string;
    attribute2?: string;
    attribute3?: string;
}

export interface EventParams {
    offset?: number;
    limit?: number;
    userId?: number;
    operationType?: number;
    fromDate?: string;
    toDate?: string;
    status?: number;
}

export interface OperationType {
    id: number;
    name: string;
}

export interface OperationTypeList {
    operations: OperationType[];
}