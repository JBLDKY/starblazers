import { get } from 'svelte/store';
import { jwtStore } from '../store/auth';
import type { GameConnection } from './gcm';

export class WebSocketManager {
	private ws: WebSocket | null = null;
	public messages: string[];
	private setSynchronizedState: (state: SynchronizeStateMessage) => void;

	constructor(
		setGameStateData: (state?: any) => void,
		setSynchronizedState: (message?: SynchronizeStateMessage) => void,
		gcm: GameConnection
	) {
		console.log('gcm.socket');
		console.log(gcm.socket);
		this.ws = gcm.socket;
		this.messages = [];
		this.setGameStateData = setGameStateData;
		this.setSynchronizedState = setSynchronizedState;
	}

	getMessages() {
		return this.messages;
	}

	disconnect() {
		if (this.ws) {
			this.ws.close();
			this.ws = null;
		}
	}

	handleReceivedWebSocketData(data: BaseWebSocketMessage) {
		switch (data.type) {
			case 'SynchronizeState':
				this.setSynchronizedState(data as SynchronizeStateMessage);
		}
	}

	sendMessage(data: BaseWebSocketMessage) {
		if (this.ws === null || this.ws.readyState !== WebSocket.OPEN) {
			console.error('WebSocket is not connected');
			this.close();
			this.ws = null;
			return;
		}

		try {
			this.ws.send(JSON.stringify(data));
		} catch (error) {
			console.log('caught error, reconnecting');
			this.close();
			this.ws = null;
			// this.connect();
		}
	}
	reconnect() {
		// setTimeout(() => {
		// 	console.log('Reconnecting to WebSocket...');
		// 	this.connect();
		// }, 5000); // Attempt to reconnect every 5 seconds
	}

	close() {
		if (this.ws) {
			this.ws.close();
			this.ws = null;
		}
	}

	isOk(): boolean {
		return Boolean(this.ws !== null && this.ws.readyState == 1);
	}
}

interface BaseWebSocketMessage {
	type: string;
}

interface AuthMessage extends BaseWebSocketMessage {
	jwt: string;
}

interface SynchronizeStateMessage extends BaseWebSocketMessage {
	state: UserState;
}

interface UserState {
	player_id?: string; // UUID
	lobby_id?: string; // UUID
	game_id?: string; // UUID
}
