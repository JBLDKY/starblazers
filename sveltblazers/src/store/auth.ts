import { localStorageStore } from '@skeletonlabs/skeleton';
import type { Writable } from 'svelte/store';

export const jwtStore: Writable<string> = localStorageStore('token', '');

