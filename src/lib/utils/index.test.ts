import { describe, it, expect, vi, beforeEach } from 'vitest';
import { toReadableSize, formatUTCDateTime, formatUTCDateTimeShort, createToastSettings, handleNodeNavigation } from './index';
import { ToastType } from '$lib/models';
import { goto } from '$app/navigation';

describe('toReadableSize', () => {
  it('should return 0 B for size 0', () => {
    expect(toReadableSize(0)).toBe('0 B');
  });

  it('should handle byte sizes properly', () => {
    expect(toReadableSize(100)).toBe('100 B');
    expect(toReadableSize(999)).toBe('999 B');
  });

  it('should handle kilobyte sizes properly', () => {
    expect(toReadableSize(1024)).toBe('1 KB');
    expect(toReadableSize(1536)).toBe('1.5 KB');
    expect(toReadableSize(10240)).toBe('10 KB');
  });

  it('should handle megabyte sizes properly', () => {
    expect(toReadableSize(1048576)).toBe('1 MB');
    expect(toReadableSize(1572864)).toBe('1.5 MB');
    expect(toReadableSize(10485760)).toBe('10 MB');
  });

  it('should handle gigabyte sizes properly', () => {
    expect(toReadableSize(1073741824)).toBe('1 GB');
    expect(toReadableSize(1610612736)).toBe('1.5 GB');
    expect(toReadableSize(10737418240)).toBe('10 GB');
  });

  it('should handle terabyte sizes properly', () => {
    expect(toReadableSize(1099511627776)).toBe('1 TB');
    expect(toReadableSize(1649267441664)).toBe('1.5 TB');
  });

  it('should handle petabyte sizes properly', () => {
    expect(toReadableSize(1125899906842624)).toBe('1 PB');
  });

  it('should handle sizes with decimal places correctly', () => {
    expect(toReadableSize(1126)).toBe('1.1 KB');
    expect(toReadableSize(1178)).toBe('1.2 KB'); // Updated to match actual implementation
    expect(toReadableSize(1433)).toBe('1.4 KB');
  });

  it('should not exceed the maximum unit', () => {
    const extremelyLargeNumber = Number.MAX_SAFE_INTEGER;
    expect(toReadableSize(extremelyLargeNumber)).not.toContain('undefined');
    expect(toReadableSize(extremelyLargeNumber)).toContain('PB');
  });
});

describe('formatUTCDateTime', () => {
  it('should format a valid date string correctly', () => {
    const result = formatUTCDateTime('2023-05-15T14:30:45Z');
    
    // Since the exact format depends on the locale used during testing,
    // we'll check that it contains the expected parts
    expect(result).toContain('2023');
    expect(result).toContain('May');
    expect(result).toContain('15');
    expect(result).toMatch(/\d{1,2}:\d{2}:\d{2}/); // time component
    expect(result).toContain('UTC');
  });

  it('should handle invalid date strings without throwing errors', () => {
    expect(() => formatUTCDateTime('not-a-date')).not.toThrow();
    const result = formatUTCDateTime('not-a-date');
    expect(result).toBeTruthy(); // Should return something, even for invalid dates
  });

  it('should format a date with time zone information', () => {
    const result = formatUTCDateTime('2023-05-15T14:30:45+02:00');
    expect(result).toContain('UTC');
  });
});

describe('formatUTCDateTimeShort', () => {
  it('should format a valid date string correctly without time', () => {
    const result = formatUTCDateTimeShort('2023-05-15T14:30:45Z');
    
    // Should contain date components but no time
    expect(result).toContain('2023');
    expect(result).toContain('May');
    expect(result).toContain('15');
    expect(result).not.toMatch(/\d{1,2}:\d{2}:\d{2}/); // No time component
    expect(result).not.toContain('UTC');
  });

  it('should handle invalid date strings without throwing errors', () => {
    expect(() => formatUTCDateTimeShort('not-a-date')).not.toThrow();
    const result = formatUTCDateTimeShort('not-a-date');
    expect(result).toBeTruthy(); // Should return something, even for invalid dates
  });
});

describe('createToastSettings', () => {
  it('should create toast settings with success type', () => {
    const message = 'Operation successful';
    const settings = createToastSettings(message, ToastType.Success);
    
    expect(settings.message).toBe(message);
    expect(settings.background).toBe('variant-filled-success');
    expect(settings.timeout).toBe(3000);
    expect(settings.autohide).toBe(true);
  });

  it('should create toast settings with error type', () => {
    const message = 'Operation failed';
    const settings = createToastSettings(message, ToastType.Error);
    
    expect(settings.message).toBe(message);
    expect(settings.background).toBe('variant-filled-error');
    expect(settings.timeout).toBe(3000);
    expect(settings.autohide).toBe(true);
  });

  it('should create toast settings with warning type', () => {
    const message = 'Proceed with caution';
    const settings = createToastSettings(message, ToastType.Warning);
    
    expect(settings.message).toBe(message);
    expect(settings.background).toBe('variant-filled-warning');
    expect(settings.timeout).toBe(3000);
    expect(settings.autohide).toBe(true);
  });

  it('should create toast settings with info type', () => {
    const message = 'Information message';
    const settings = createToastSettings(message, ToastType.Info);
    
    expect(settings.message).toBe(message);
    expect(settings.background).toBe('variant-filled-primary');
    expect(settings.timeout).toBe(3000);
    expect(settings.autohide).toBe(true);
  });

  it('should respect custom timeout value', () => {
    const message = 'Custom timeout';
    const customTimeout = 5000;
    const settings = createToastSettings(message, ToastType.Info, customTimeout);
    
    expect(settings.timeout).toBe(customTimeout);
  });
});

describe('handleNodeNavigation', () => {
  beforeEach(() => {
    vi.mocked(goto).mockClear();
  });

  it('should navigate to node details page when permissions is not provided', async () => {
    const nodeId = 123;
    await handleNodeNavigation(nodeId);
    
    expect(goto).toHaveBeenCalledTimes(1);
    expect(goto).toHaveBeenCalledWith(`/nodes/${nodeId}`);
  });

  it('should navigate to node details page when permissions is false', async () => {
    const nodeId = 123;
    await handleNodeNavigation(nodeId, false);
    
    expect(goto).toHaveBeenCalledTimes(1);
    expect(goto).toHaveBeenCalledWith(`/nodes/${nodeId}`);
  });

  it('should navigate to node permissions page when permissions is true', async () => {
    const nodeId = 123;
    await handleNodeNavigation(nodeId, true);
    
    expect(goto).toHaveBeenCalledTimes(1);
    expect(goto).toHaveBeenCalledWith(`/nodes/${nodeId}/permissions`);
  });
});
