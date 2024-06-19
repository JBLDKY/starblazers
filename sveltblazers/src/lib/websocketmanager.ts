import { get } from 'svelte/store';
import { jwtStore } from '../store/auth';

export class WebSocketManager {
	private url: string;
	private ws: WebSocket | null = null;
	public messages: string[];
	private setGameStateData: (state: any) => void;

	constructor(setGameStateData: (state: any) => void) {
		this.url = 'ws://localhost:3030/lobby';
		this.messages = [];
		this.setGameStateData = setGameStateData;
	}

	getMessages() {
		return this.messages;
	}

	connect() {
		this.ws = new WebSocket(this.url);

		this.ws.onopen = () => {
			const jwt = get(jwtStore);
			this.ws.send(JSON.stringify({ type: 'Auth', jwt: jwt }));
		};

		this.ws.onmessage = (event) => {
			let data;
			console.log(event);
			try {
				data = JSON.parse(event.data);
				this.setGameStateData(data);
			} catch (error) {
				console.error('couldnt parse data received from websocket into json');
			}
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

	sendMessage(data: any) {
		if (!this.ws || this.ws.readyState !== WebSocket.OPEN) {
			console.error('WebSocket is not connected');
			return;
		}

		try {
			this.ws.send(data);
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
