import { get } from 'svelte/store';
import { jwtStore } from '../store/auth';

export class WebSocketManager {
	private url: string;
	private ws: WebSocket | null = null;
	public messages: string[];
	private messageHandlers: { [key: string]: (data: any) => void } = {};

	constructor() {
		this.url = 'ws://localhost:3030/lobby';
		this.messages = [];
	}

	getMessages() {
		return this.messages;
	}

	connect() {
		this.ws = new WebSocket(this.url);

		this.ws.onopen = () => {
			const jwt = get(jwtStore);
			console.log('lobby connection established');
			console.log(jwt);
			this.ws.send(JSON.stringify({ type: 'auth', jwt: jwt }));
		};

		this.ws.onmessage = (event) => {
			// create callback?
			// this.messages.push(event.data);
			const handler = this.messageHandlers[event.type];
			if (handler) {
				handler(event.data);
			} else {
				console.warn(`No handler for message type: ${event.type}`);
			}
		};

		this.ws.onclose = (event) => {
			console.log('WebSocket connection closed', event.code, event.reason);
		};

		this.ws.onerror = (error) => {
			console.error('WebSocket error', error);
		};
	}

	onMessage(type: string, handler: (data: any) => void) {
		this.messageHandlers[type] = handler;
	}

	sendMessage(data: any) {
		console.log('sending!');

		if (!this.ws || !this.ws.readyState === WebSocket.OPEN) {
			console.error('WebSocket is not connected');
			return;
		}

		console.log(data);
		this.ws.send(data);
	}

	close() {
		if (this.ws) {
			this.ws.close();
		}
	}
}
