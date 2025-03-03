import { describe, it, expect, vi, beforeEach } from 'vitest';
import { invoke } from '@tauri-apps/api/core';
import { initAuthCodeFlow, connect } from './index';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}));

describe('Auth Service', () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  describe('initAuthCodeFlow', () => {
    it('should call the tauri invoke with the correct URL', async () => {
      vi.mocked(invoke).mockResolvedValue(true);
      
      const url = 'https://example.com';
      const result = await initAuthCodeFlow(url);
      
      expect(invoke).toHaveBeenCalledWith('init_auth_code_flow', { url });
      expect(result).toBe(true);
    });

    it('should return false when refresh token is not available', async () => {
      vi.mocked(invoke).mockResolvedValue(false);
      
      const url = 'https://example.com';
      const result = await initAuthCodeFlow(url);
      
      expect(result).toBe(false);
    });

    it('should throw an error when the invoke call fails', async () => {
      vi.mocked(invoke).mockRejectedValue(new Error('Auth error'));
      
      const url = 'https://example.com';
      await expect(initAuthCodeFlow(url)).rejects.toThrow('Auth error');
    });
  });

  describe('connect', () => {
    it('should call connect without auth code when using refresh token', async () => {
      const mockUserAccount = {
        id: 123,
        firstName: 'Test',
        lastName: 'User',
        userName: 'testuser',
        email: 'test@example.com',
        isConfigManager: true,
        isRoomManager: false,
        isUserManager: true,
        isGroupManager: false,
        isAuditor: true,
        isCloud: false
      };
      vi.mocked(invoke).mockResolvedValue(mockUserAccount);
      
      const result = await connect(true);
      
      expect(invoke).toHaveBeenCalledWith('connect');
      expect(result).toEqual(mockUserAccount);
    });

    it('should call connect with auth code when not using refresh token', async () => {
      const mockUserAccount = {
        id: 123,
        firstName: 'Test',
        lastName: 'User',
        userName: 'testuser',
        email: 'test@example.com',
        isConfigManager: true,
        isRoomManager: false,
        isUserManager: true,
        isGroupManager: false,
        isAuditor: true,
        isCloud: false
      };
      vi.mocked(invoke).mockResolvedValue(mockUserAccount);
      
      const authCode = '12345';
      const result = await connect(false, authCode);
      
      expect(invoke).toHaveBeenCalledWith('connect', { authCode });
      expect(result).toEqual(mockUserAccount);
    });

    it('should throw an error when the invoke call fails', async () => {
      vi.mocked(invoke).mockRejectedValue(new Error('Connect error'));
      
      await expect(connect(true)).rejects.toThrow('Connect error');
    });
  });
});