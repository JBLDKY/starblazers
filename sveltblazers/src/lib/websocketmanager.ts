export class WebSocketManager {
	private url: string;
	private ws: WebSocket | null = null;
	public messages: string[];
	private messageHandlers: { [key: string]: (data: any) => void } = {};

	constructor() {
		this.url = 'ws://localhost:3030/ws';
		this.messages = [];
	}

	getMessages() {
		return this.messages;
	}

	connect() {
		this.ws = new WebSocket(this.url);

		this.ws.onopen = () => {
			console.log('WebSocket connection established');
		};

		this.ws.onmessage = (event) => {
			// create callback?
			this.messages.push(event.data);
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

	sendMessage(type: string, data: any) {
		if (this.ws && this.ws.readyState === WebSocket.OPEN) {
			const message = JSON.stringify({ type, data });
			this.ws.send(message);
		} else {
			console.error('WebSocket is not connected');
		}
	}

	close() {
		if (this.ws) {
			this.ws.close();
		}
	}
}
