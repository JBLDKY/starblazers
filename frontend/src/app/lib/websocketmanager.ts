export class WebSocketManager {
	private url: string;
	private ws: WebSocket | null = null;
	public messages: string[];

	constructor() {
		this.url = "ws://localhost:3030/chat";
		this.messages = [];
	}

	getMessages() {
		return this.messages;
	}

	connect() {
		this.ws = new WebSocket(this.url);

		this.ws.onopen = () => {
			console.log("WebSocket connection established");
		};

		this.ws.onmessage = (event) => {
			// create callback?
			this.messages.push(event.data);
		};

		this.ws.onclose = (event) => {
			console.log("WebSocket connection closed", event.code, event.reason);
		};

		this.ws.onerror = (error) => {
			console.error("WebSocket error", error);
		};
	}

	sendMessage(message: string) {
		if (this.ws && this.ws.readyState === WebSocket.OPEN) {
			this.messages.push(message);
			this.ws.send(message);
		} else {
			console.error("WebSocket is not connected");
		}
	}

	close() {
		if (this.ws) {
			this.ws.close();
		}
	}
}
