import { jwtStore, playerInfoStore } from '../store/auth';
import { get } from 'svelte/store';

export type PublicPlayerData = {
	authority: string;
	email: string;
	username: string;
	uuid: string;
};

export const lobbyName = (playerInfo: PublicPlayerData): string => {
	return playerInfo.username + "'s lobby";
};

export async function get_player_info(): Promise<PublicPlayerData> {
	const res = await fetch('http://localhost:3030/players/player', {
		method: 'GET',
		headers: {
			authorization: get(jwtStore)
		}
	});

	if (res.ok) {
		const player_info = await res.json();
		playerInfoStore.set(player_info);
		return player_info;
	} else {
		throw new Error(await res.text());
	}
}
