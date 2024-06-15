import { jwtStore } from '../store/auth';
import { get } from 'svelte/store';

export interface PublicPlayerData {
	authority: string;
	email: string;
	username: string;
	uuid: string;
}
export async function get_player_info(): Promise<PublicPlayerData> {
	const res = await fetch('http://localhost:3030/players/player', {
		method: 'GET',
		headers: {
			authorization: get(jwtStore)
		}
	});

	if (res.ok) {
		return res.json();
	} else {
		throw new Error(await res.text());
	}
}
