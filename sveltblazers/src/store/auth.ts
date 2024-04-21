import {  writable } from 'svelte/store';
import { localStorageStore } from '@skeletonlabs/skeleton';
import type { Writable } from 'svelte/store';

export const isAuthenticated = writable(false);
export const authToken = writable("");

export function setAuth(token: string) {
    isAuthenticated.set(true);
    authToken.set(token);
    localStorage.setItem('jwt', token);  // Optionally store the token in localStorage
}

export function clearAuth() {
    isAuthenticated.set(false);
    authToken.set("");
    localStorage.removeItem('jwt');  // Clear the token from localStorage
}


export const jwtStore: Writable<string> = localStorageStore('token', '');
