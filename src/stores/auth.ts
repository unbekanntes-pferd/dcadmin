import type { UserAccount } from '$lib/auth/models';
import { writable } from 'svelte/store';

export const isLoggedIn = writable(false);
export const userAccount = writable<UserAccount | null>(null);

export function login() {
  isLoggedIn.set(true);
}

export function logout() {
  isLoggedIn.set(false);
}

export function setUserAccount(account: UserAccount) {
  userAccount.set(account);
}

export function clearUserAccount() {
  userAccount.set(null);
}
