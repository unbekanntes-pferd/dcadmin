import { describe, it, expect, vi, beforeEach } from 'vitest';
import { invoke } from '@tauri-apps/api/core';
import { getCustomerInfo } from './index';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}));

describe('Customer Service', () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  describe('getCustomerInfo', () => {
    it('should call the tauri invoke and return customer info', async () => {
      // Setup mock response
      const mockCustomerInfo = {
        userCount: 100,
        userLimit: 250,
        spaceUsed: 1073741824, // 1GB
        spaceLimit: 10737418240 // 10GB
      };
      vi.mocked(invoke).mockResolvedValue(mockCustomerInfo);
      
      // Call the function
      const result = await getCustomerInfo();
      
      // Assertions
      expect(invoke).toHaveBeenCalledWith('get_customer_info');
      expect(result).toEqual(mockCustomerInfo);
    });

    it('should throw an error when the invoke call fails', async () => {
      vi.mocked(invoke).mockRejectedValue(new Error('API error'));
      
      await expect(getCustomerInfo()).rejects.toThrow('API error');
    });
  });
});