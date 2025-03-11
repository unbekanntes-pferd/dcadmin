import { describe, it, expect, vi, beforeEach } from 'vitest';
import { invoke } from '@tauri-apps/api/core';
import { getEvents, downloadEvents, getOperationTypes } from './index';
import type { EventParams } from './models';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}));

describe('Events Service', () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  describe('getEvents', () => {
    it('should call the tauri invoke with the correct parameters', async () => {
      // Setup mock response
      const mockEvents = {
        events: [{ id: 1, message: 'Test event' }],
        range: { offset: 0, limit: 10, total: 1 }
      };
      vi.mocked(invoke).mockResolvedValue(mockEvents);
      
      // Call the function
      const params: EventParams = { 
        offset: 0, 
        limit: 10,
        fromDate: '2023-01-01T00:00:00Z',
        toDate: '2023-01-31T23:59:59Z'
      };
      const result = await getEvents(params);
      
      // Assertions
      expect(invoke).toHaveBeenCalledWith('get_events', { params });
      expect(result).toEqual(mockEvents);
    });

    it('should throw an error when the invoke call fails', async () => {
      vi.mocked(invoke).mockRejectedValue(new Error('API error'));
      
      const params: EventParams = { offset: 0, limit: 10 };
      await expect(getEvents(params)).rejects.toThrow('API error');
    });
  });

  describe('downloadEvents', () => {
    it('should call the tauri invoke with the correct parameters', async () => {
      vi.mocked(invoke).mockResolvedValue(undefined);
      
      const path = '/test/events.csv';
      const params: EventParams = { 
        offset: 0, 
        limit: 500,
        fromDate: '2023-01-01T00:00:00Z',
        toDate: '2023-01-31T23:59:59Z'
      };
      await downloadEvents(path, params);
      
      expect(invoke).toHaveBeenCalledWith('export_events', { path, params });
    });

    it('should throw an error when the invoke call fails', async () => {
      vi.mocked(invoke).mockRejectedValue(new Error('Export error'));
      
      const path = '/test/events.csv';
      const params: EventParams = { offset: 0, limit: 500 };
      await expect(downloadEvents(path, params)).rejects.toThrow('Export error');
    });
  });

  describe('getOperationTypes', () => {
    it('should call the tauri invoke and return operation types', async () => {
      const mockOperationTypes = {
        operations: [
          { id: 1, name: 'login' },
          { id: 2, name: 'logout' }
        ]
      };
      vi.mocked(invoke).mockResolvedValue(mockOperationTypes);
      
      const result = await getOperationTypes();
      
      expect(invoke).toHaveBeenCalledWith('get_operation_types');
      expect(result).toEqual(mockOperationTypes);
    });

    it('should throw an error when the invoke call fails', async () => {
      vi.mocked(invoke).mockRejectedValue(new Error('API error'));
      
      await expect(getOperationTypes()).rejects.toThrow('API error');
    });
  });
});