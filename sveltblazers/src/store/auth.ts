import { localStorageStore } from '@skeletonlabs/skeleton';
import type { Writable } from 'svelte/store';
import type { PublicPlayerData } from '../routes/helpers';

export const jwtStore: Writable<string> = localStorageStore('token', '');

export const playerInfoStore: Writable<PublicPlayerData> = localStorageStore('playerInfo', {
	authority: '',
	email: '',
	username: '',
	uuid: ''
});
