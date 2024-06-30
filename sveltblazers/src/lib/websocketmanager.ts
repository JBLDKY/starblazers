import { get } from 'svelte/store';
import { jwtStore } from '../store/auth';

export class WebSocketManager {
	private url: string;
	private ws: WebSocket | null = null;
	public messages: string[];
	private setGameStateData: (state: any) => void;
	private setSynchronizedState: (state: SynchronizeStateMessage) => void;

	constructor(
		setGameStateData: (state: any) => void,
		setSynchronizedState: (state: SynchronizeStateMessage) => void
	) {
		this.url = 'ws://localhost:3030/lobby';
		this.messages = [];
		this.setGameStateData = setGameStateData;
		this.setSynchronizedState = setSynchronizedState;
	}

	getMessages() {
		return this.messages;
	}

	connect() {
		document.cookie = 'Authorization=' + get(jwtStore) + '; path=/';

		this.ws = new WebSocket(this.url);

		this.ws.onopen = () => {
			const jwt = get(jwtStore);
			this.ws.send(JSON.stringify({ type: 'Auth', jwt: jwt }));
		};

		this.ws.onmessage = (event) => {
			let data;
			try {
				data = JSON.parse(event.data);
			} catch (error) {
				console.log(event.data);
				console.error('couldnt parse data received from websocket into json');
				return;
			}

			this.handleReceivedWebSocketData(data);
		};

		this.ws.onclose = (event) => {
			console.log('WebSocket connection closed', event.code, event.reason);
			this.reconnect();
		};

		this.ws.onerror = (error) => {
			this.ws = new WebSocket(this.url);

			console.error('WebSocket error', error);
		};
	}

	handleReceivedWebSocketData(data: BaseWebSocketMessage) {
		switch (data.type) {
			case 'SynchronizeState':
				this.setSynchronizedState(data as SynchronizeStateMessage);
		}
	}

	sendMessage(data: BaseWebSocketMessage) {
		if (!this.ws || this.ws.readyState !== WebSocket.OPEN) {
			console.error('WebSocket is not connected');
			return;
		}

		try {
			this.ws.send(JSON.stringify(data));
		} catch (error) {
			console.log('caught error, reconnecting');
			this.connect();
		}
	}
	reconnect() {
		setTimeout(() => {
			console.log('Reconnecting to WebSocket...');
			this.connect();
		}, 5000); // Attempt to reconnect every 5 seconds
	}

	close() {
		if (this.ws) {
			this.ws.close();
		}
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

// You can extend this with more specific message types as needed
