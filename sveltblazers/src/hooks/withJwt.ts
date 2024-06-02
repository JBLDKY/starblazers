import { VERIFY_JWT_SERVER_URL } from '../constants';
import { get } from 'svelte/store';
import { jwtStore } from '../store/auth';

export const validateJwt = async () => {
	const jwt = get(jwtStore);

	try {
		const response = await fetch(VERIFY_JWT_SERVER_URL, {
			method: 'POST',
			mode: 'cors', // no-cors, *cors, same-origin
			headers: {
				'Content-Type': 'application/json',
				Authorization: jwt
			}
		});

		if (!response.ok) {
			switch (response.status) {
				case 401:
					console.warn('Token has been tampered with or has expired');
					throw new Error('Unauthorized');
				default:
					console.error('Server unavailable');
					throw new Error('Server unavailable');
			}
		}
	} catch (error) {
		console.error('Fetch error:', error);
		throw error; // Re-throw the error to be caught by the caller
	}
};
