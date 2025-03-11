import { writable } from "svelte/store";

export const lastUserTab = writable(0);
export const lastUserListPage = writable(0);
export const lastUserListLimit = writable(10);