import { describe, it, expect, vi, beforeEach } from 'vitest';
import { invoke } from '@tauri-apps/api/core';
import { 
  getPermissions, 
  downloadUserPermissions, 
  downloadAllUserPermissions,
  displayPermissionsTemplate,
  PermissionsTemplate
} from './index';
import type { NodePermissions } from './models';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}));

describe('Permissions Service', () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  describe('getPermissions', () => {
    it('should call the tauri invoke with the correct parameters', async () => {
      // Setup mock response
      const mockPermissionsList = [
        { 
          nodeId: 1, 
          nodeName: 'Test Node', 
          nodeCreatedById: 123,
          userPermissions: [] 
        },
        { 
          nodeId: 2, 
          nodeName: 'Another Node',
          nodeCreatedById: 0, // This should be filtered out
          userPermissions: []
        }
      ];
      vi.mocked(invoke).mockResolvedValue(mockPermissionsList);
      
      // Call the function
      const params = { filter: 'test' };
      const result = await getPermissions(params);
      
      // Assertions
      expect(invoke).toHaveBeenCalledWith('get_permissions', { params });
      expect(result).toHaveLength(1);
      expect(result[0].nodeId).toBe(1);
    });

    it('should throw an error when the invoke call fails', async () => {
      vi.mocked(invoke).mockRejectedValue(new Error('API error'));
      
      const params = { filter: 'test' };
      await expect(getPermissions(params)).rejects.toThrow('API error');
    });
  });

  describe('downloadUserPermissions', () => {
    it('should call the tauri invoke with the correct parameters', async () => {
      vi.mocked(invoke).mockResolvedValue(undefined);
      
      const path = '/test/path.csv';
      const params = { filter: 'userId:eq:123' };
      await downloadUserPermissions(path, params);
      
      expect(invoke).toHaveBeenCalledWith('export_user_permissions', { path, params });
    });

    it('should throw an error when the invoke call fails', async () => {
      vi.mocked(invoke).mockRejectedValue(new Error('Export error'));
      
      const path = '/test/path.csv';
      const params = { filter: 'userId:eq:123' };
      await expect(downloadUserPermissions(path, params)).rejects.toThrow('Export error');
    });
  });

  describe('downloadAllUserPermissions', () => {
    it('should call the tauri invoke with the correct parameters', async () => {
      vi.mocked(invoke).mockResolvedValue(undefined);
      
      const path = '/test/path.csv';
      await downloadAllUserPermissions(path);
      
      expect(invoke).toHaveBeenCalledWith('export_all_user_permissions', { path });
    });

    it('should throw an error when the invoke call fails', async () => {
      vi.mocked(invoke).mockRejectedValue(new Error('Export all error'));
      
      const path = '/test/path.csv';
      await expect(downloadAllUserPermissions(path)).rejects.toThrow('Export all error');
    });
  });

  describe('displayPermissionsTemplate', () => {
    it('should return None for null or undefined permissions', () => {
      expect(displayPermissionsTemplate(undefined as any)).toBe(PermissionsTemplate.None);
      expect(displayPermissionsTemplate(null as any)).toBe(PermissionsTemplate.None);
    });

    it('should identify Room Administrator permissions', () => {
      const roomAdminPerms: NodePermissions = {
        manage: true,
        read: true,
        change: true,
        delete: true,
        create: true,
        manageDownloadShare: true,
        manageUploadShare: true,
        readRecycleBin: true,
        restoreRecycleBin: true,
        deleteRecycleBin: true
      };
      
      expect(displayPermissionsTemplate(roomAdminPerms)).toBe(PermissionsTemplate.RoomAdministator);
    });

    it('should identify Edit permissions', () => {
      const editPerms: NodePermissions = {
        manage: false,
        read: true,
        change: true,
        delete: true,
        create: true,
        manageDownloadShare: true,
        manageUploadShare: true,
        readRecycleBin: true,
        restoreRecycleBin: true,
        deleteRecycleBin: false
      };
      
      expect(displayPermissionsTemplate(editPerms)).toBe(PermissionsTemplate.Edit);
    });

    it('should identify Read permissions', () => {
      const readPerms: NodePermissions = {
        manage: false,
        read: true,
        change: false,
        delete: false,
        create: false,
        manageDownloadShare: true,
        manageUploadShare: false,
        readRecycleBin: false,
        restoreRecycleBin: false,
        deleteRecycleBin: false
      };
      
      expect(displayPermissionsTemplate(readPerms)).toBe(PermissionsTemplate.Read);
    });

    it('should identify Custom permissions', () => {
      const customPerms: NodePermissions = {
        manage: false,
        read: true,
        change: true,
        delete: false,
        create: true,
        manageDownloadShare: false,
        manageUploadShare: true,
        readRecycleBin: false,
        restoreRecycleBin: false,
        deleteRecycleBin: false
      };
      
      expect(displayPermissionsTemplate(customPerms)).toBe(PermissionsTemplate.Custom);
    });
  });
});