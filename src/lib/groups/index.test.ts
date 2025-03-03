import { describe, it, expect, vi, beforeEach } from 'vitest';
import { invoke } from '@tauri-apps/api/core';
import { 
  getGroups, 
  getGroup, 
  downloadGroups, 
  getGroupUsers, 
  downloadGroupUsers,
  downloadAllGroupUsers
} from './index';
import type { ListParams } from '$lib/models';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}));

describe('Groups Service', () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  describe('getGroups', () => {
    it('should call the tauri invoke with the correct parameters', async () => {
      // Setup mock response
      const mockGroups = {
        items: [
          { 
            id: 1,
            name: 'Admins',
            cntUsers: 5,
            cntChildGroups: 2
          }
        ],
        range: { offset: 0, limit: 10, total: 1 }
      };
      vi.mocked(invoke).mockResolvedValue(mockGroups);
      
      // Call the function
      const params: ListParams = { 
        offset: 0, 
        limit: 10,
        filter: 'name:cn:admin'
      };
      const result = await getGroups(params);
      
      // Assertions
      expect(invoke).toHaveBeenCalledWith('get_groups', { params });
      expect(result).toEqual(mockGroups);
    });

    it('should throw an error when the invoke call fails', async () => {
      vi.mocked(invoke).mockRejectedValue(new Error('API error'));
      
      const params: ListParams = { offset: 0, limit: 10 };
      await expect(getGroups(params)).rejects.toThrow('API error');
    });
  });

  describe('getGroup', () => {
    it('should call the tauri invoke with the correct parameter', async () => {
      const mockGroup = { 
        id: 1,
        name: 'Admins',
        cntUsers: 5
      };
      vi.mocked(invoke).mockResolvedValue(mockGroup);
      
      const groupId = 1;
      const result = await getGroup(groupId);
      
      expect(invoke).toHaveBeenCalledWith('get_group', { groupId });
      expect(result).toEqual(mockGroup);
    });

    it('should throw an error when the invoke call fails', async () => {
      vi.mocked(invoke).mockRejectedValue(new Error('API error'));
      
      const groupId = 999; // Non-existent group
      await expect(getGroup(groupId)).rejects.toThrow('API error');
    });
  });

  describe('downloadGroups', () => {
    it('should call the tauri invoke with the correct parameters', async () => {
      vi.mocked(invoke).mockResolvedValue(undefined);
      
      const path = '/test/groups.csv';
      const params: ListParams = { 
        offset: 0, 
        limit: 500
      };
      await downloadGroups(params, path);
      
      expect(invoke).toHaveBeenCalledWith('export_groups', { params, path });
    });

    it('should throw an error when the invoke call fails', async () => {
      vi.mocked(invoke).mockRejectedValue(new Error('Export error'));
      
      const path = '/test/groups.csv';
      const params: ListParams = { offset: 0, limit: 500 };
      await expect(downloadGroups(params, path)).rejects.toThrow('Export error');
    });
  });

  describe('getGroupUsers', () => {
    it('should call the tauri invoke with the correct parameters', async () => {
      const mockGroupUsers = {
        items: [
          {
            id: 101,
            firstName: 'John',
            lastName: 'Doe',
            userName: 'johndoe'
          }
        ],
        range: { offset: 0, limit: 10, total: 1 }
      };
      vi.mocked(invoke).mockResolvedValue(mockGroupUsers);
      
      const groupId = 1;
      const params: ListParams = { offset: 0, limit: 10 };
      const result = await getGroupUsers(groupId, params);
      
      expect(invoke).toHaveBeenCalledWith('get_group_users', { groupId, params });
      expect(result).toEqual(mockGroupUsers);
    });

    it('should throw an error when the invoke call fails', async () => {
      vi.mocked(invoke).mockRejectedValue(new Error('API error'));
      
      const groupId = 1;
      const params: ListParams = { offset: 0, limit: 10 };
      await expect(getGroupUsers(groupId, params)).rejects.toThrow('API error');
    });
  });

  describe('downloadGroupUsers', () => {
    it('should call the tauri invoke with the correct parameters', async () => {
      vi.mocked(invoke).mockResolvedValue(undefined);
      
      const groupId = 1;
      const path = '/test/group_users.csv';
      const params: ListParams = { offset: 0, limit: 500 };
      await downloadGroupUsers(groupId, params, path);
      
      expect(invoke).toHaveBeenCalledWith('export_group_users', { groupId, params, path });
    });

    it('should throw an error when the invoke call fails', async () => {
      vi.mocked(invoke).mockRejectedValue(new Error('Export error'));
      
      const groupId = 1;
      const path = '/test/group_users.csv';
      const params: ListParams = { offset: 0, limit: 500 };
      await expect(downloadGroupUsers(groupId, params, path)).rejects.toThrow('Export error');
    });
  });

  describe('downloadAllGroupUsers', () => {
    it('should call the tauri invoke with the correct parameter', async () => {
      vi.mocked(invoke).mockResolvedValue(undefined);
      
      const path = '/test/all_group_users.csv';
      await downloadAllGroupUsers(path);
      
      expect(invoke).toHaveBeenCalledWith('export_all_group_users', { path });
    });

    it('should throw an error when the invoke call fails', async () => {
      vi.mocked(invoke).mockRejectedValue(new Error('Export error'));
      
      const path = '/test/all_group_users.csv';
      await expect(downloadAllGroupUsers(path)).rejects.toThrow('Export error');
    });
  });
});