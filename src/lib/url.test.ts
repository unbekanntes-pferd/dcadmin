import { describe, it, expect, vi, beforeEach } from 'vitest';
import { validateUrl, addHttps } from './url';
import { invoke } from '@tauri-apps/api/core';

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}));

describe('URL Utilities', () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  describe('addHttps', () => {
    it('should add https:// prefix to URLs without protocol', () => {
      expect(addHttps('example.com')).toBe('https://example.com');
      expect(addHttps('www.example.com')).toBe('https://www.example.com');
      expect(addHttps('sub.example.com')).toBe('https://sub.example.com');
    });

    it('should replace http:// with https://', () => {
      expect(addHttps('http://example.com')).toBe('https://example.com');
      expect(addHttps('http://www.example.com')).toBe('https://www.example.com');
    });

    it('should not modify URLs that already have https://', () => {
      expect(addHttps('https://example.com')).toBe('https://example.com');
      expect(addHttps('https://www.example.com')).toBe('https://www.example.com');
    });

    it('should handle URLs with paths and query parameters', () => {
      expect(addHttps('example.com/path')).toBe('https://example.com/path');
      expect(addHttps('http://example.com/path')).toBe('https://example.com/path');
      expect(addHttps('example.com/path?query=value')).toBe('https://example.com/path?query=value');
    });

    it('should handle empty strings', () => {
      expect(addHttps('')).toBe('https://');
    });
  });

  describe('validateUrl', () => {
    it('should return true for valid URLs when DRACOON check passes', async () => {
      // Mock the invoke function to return true for checkDracoonUrl
      vi.mocked(invoke).mockResolvedValue(true);
      
      expect(await validateUrl('https://example.com')).toBe(true);
      expect(await validateUrl('https://www.example.com')).toBe(true);
      expect(await validateUrl('https://sub.example.com')).toBe(true);
      expect(await validateUrl('https://example.com/path')).toBe(true);
      expect(await validateUrl('https://example.com/path/to/resource')).toBe(true);
    });

    it('should return false for valid URLs when DRACOON check fails', async () => {
      // Mock the invoke function to return false for checkDracoonUrl
      vi.mocked(invoke).mockResolvedValue(false);
      
      expect(await validateUrl('https://example.com')).toBe(false);
    });

    it('should return false for invalid URLs without checking DRACOON URL', async () => {
      // Invalid URLs
      expect(await validateUrl('not-a-url')).toBe(false);
      expect(await validateUrl('http://')).toBe(false);
      expect(await validateUrl('http://.')).toBe(false);
      expect(await validateUrl('http://.com')).toBe(false);
      
      // The invoke function should not be called for invalid URLs
      expect(invoke).not.toHaveBeenCalled();
    });

    it('should pass the URL to the checkDracoonUrl function', async () => {
      const url = 'https://example.com';
      vi.mocked(invoke).mockResolvedValue(true);
      
      await validateUrl(url);
      
      expect(invoke).toHaveBeenCalledWith('validate_dracoon_url', { url });
    });
  });
});