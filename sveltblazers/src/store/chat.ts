import { writable } from 'svelte/store';

export const chatLogStore = writable<string[]>([]);
