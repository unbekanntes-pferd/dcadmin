import { describe, it, expect, vi, beforeEach } from 'vitest';
import { invoke } from '@tauri-apps/api/core';
import { getUsers, downloadUsers } from './index';
import type { ListParams } from '$lib/models';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}));

describe('Users Service', () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  describe('getUsers', () => {
    it('should call the tauri invoke with the correct parameters', async () => {
      // Setup mock response
      const mockUsers = {
        items: [
          { 
            id: 1,
            firstName: 'John',
            lastName: 'Doe',
            userName: 'johndoe',
            email: 'john@example.com'
          }
        ],
        range: { offset: 0, limit: 10, total: 1 }
      };
      vi.mocked(invoke).mockResolvedValue(mockUsers);
      
      // Call the function
      const params: ListParams = { 
        offset: 0, 
        limit: 10,
        filter: 'userName:cn:john'
      };
      const result = await getUsers(params);
      
      // Assertions
      expect(invoke).toHaveBeenCalledWith('get_users', { params });
      expect(result).toEqual(mockUsers);
    });

    it('should throw an error when the invoke call fails', async () => {
      vi.mocked(invoke).mockRejectedValue(new Error('API error'));
      
      const params: ListParams = { offset: 0, limit: 10 };
      await expect(getUsers(params)).rejects.toThrow('API error');
    });
  });

  describe('downloadUsers', () => {
    it('should call the tauri invoke with the correct parameters', async () => {
      vi.mocked(invoke).mockResolvedValue(undefined);
      
      const path = '/test/users.csv';
      const params: ListParams = { 
        offset: 0, 
        limit: 500,
        filter: 'isLocked:eq:true'
      };
      await downloadUsers(params, path);
      
      expect(invoke).toHaveBeenCalledWith('export_users', { params, path });
    });

    it('should throw an error when the invoke call fails', async () => {
      vi.mocked(invoke).mockRejectedValue(new Error('Export error'));
      
      const path = '/test/users.csv';
      const params: ListParams = { offset: 0, limit: 500 };
      await expect(downloadUsers(params, path)).rejects.toThrow('Export error');
    });
  });
});