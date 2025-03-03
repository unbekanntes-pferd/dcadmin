import { describe, it, expect } from 'vitest';
import { get } from 'svelte/store';
import { lastNodesPage } from './nodes';

describe('Nodes Store', () => {
  it('should initialize lastNodesPage to 0', () => {
    // Reset to initial state first
    lastNodesPage.set(0);
    expect(get(lastNodesPage)).toBe(0);
  });

  it('should update lastNodesPage when set', () => {
    lastNodesPage.set(5);
    expect(get(lastNodesPage)).toBe(5);
  });

  it('should update lastNodesPage with update method', () => {
    lastNodesPage.set(0); // Reset first
    lastNodesPage.update(n => n + 3);
    expect(get(lastNodesPage)).toBe(3);
  });
});