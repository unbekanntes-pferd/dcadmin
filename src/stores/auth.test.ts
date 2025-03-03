import { describe, it, expect, beforeEach } from 'vitest';
import { get } from 'svelte/store';
import { isLoggedIn, userAccount, login, logout, setUserAccount, clearUserAccount } from './auth';

describe('Auth Store', () => {
  beforeEach(() => {
    // Reset to initial state before each test
    logout();
    clearUserAccount();
  });

  it('should have initial state with user logged out and no account', () => {
    expect(get(isLoggedIn)).toBe(false);
    expect(get(userAccount)).toBe(null);
  });

  it('should set login state to true when login() is called', () => {
    login();
    expect(get(isLoggedIn)).toBe(true);
  });

  it('should set login state to false when logout() is called', () => {
    // First set to logged in
    login();
    expect(get(isLoggedIn)).toBe(true);
    
    // Then log out
    logout();
    expect(get(isLoggedIn)).toBe(false);
  });

  it('should set user account when setUserAccount() is called', () => {
    const mockAccount = {
      id: 123,
      firstName: 'Test',
      lastName: 'User',
      userName: 'testuser',
      email: 'test@example.com',
      isConfigManager: true,
      isRoomManager: true,
      isUserManager: false,
      isGroupManager: false,
      isAuditor: true,
      isCloud: false
    };
    
    setUserAccount(mockAccount);
    expect(get(userAccount)).toEqual(mockAccount);
  });

  it('should clear user account when clearUserAccount() is called', () => {
    // First set an account
    const mockAccount = {
      id: 123,
      firstName: 'Test',
      lastName: 'User',
      userName: 'testuser',
      email: 'test@example.com',
      isConfigManager: true,
      isRoomManager: true,
      isUserManager: false,
      isGroupManager: false, 
      isAuditor: true,
      isCloud: false
    };
    setUserAccount(mockAccount);
    
    // Then clear it
    clearUserAccount();
    expect(get(userAccount)).toBe(null);
  });
});