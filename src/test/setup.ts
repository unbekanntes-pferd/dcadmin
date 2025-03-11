import { vi, afterEach } from 'vitest';
import '@testing-library/jest-dom';
import { cleanup } from '@testing-library/svelte';

// Mock the Tauri API modules
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn()
}));

vi.mock('@tauri-apps/api/app', () => ({
  getVersion: vi.fn().mockResolvedValue('0.1.0')
}));

vi.mock('@tauri-apps/plugin-dialog', () => ({
  save: vi.fn().mockResolvedValue('/path/to/mock-file.csv')
}));

// Mock $app/navigation
vi.mock('$app/navigation', () => ({
  goto: vi.fn()
}));

// Clean up after each test
afterEach(() => {
  cleanup();
});
