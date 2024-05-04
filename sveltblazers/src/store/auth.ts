import { localStorageStore } from '@skeletonlabs/skeleton';
import type { Writable } from 'svelte/store';
import { VERIFY_JWT_SERVER_URL } from '../constants';
import { get } from 'svelte/store';

export const jwtStore: Writable<string> = localStorageStore('token', '');

export const checkJwt = async () => {
	const jwt = get(jwtStore);
	const response = await fetch(VERIFY_JWT_SERVER_URL, {
		method: 'POST',
		mode: 'cors', // no-cors, *cors, same-origin,
		headers: {
			'Content-Type': 'application/json',
			Authorization: `Bearer ${jwt}`
		}
	});
};
