import { writable } from 'svelte/store';

export const chatLogStore = writable<string[]>([]);
export const fontIsLoaded = writable<boolean>(false);
