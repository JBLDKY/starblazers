export class GameConnection {
	socket: WebSocket | null = null;

	constructor() {}

	connect() {
		this.disconnect();

		this.socket = new WebSocket('ws://localhost:3030/lobby');

		this.socket.onopen = () => {
			console.log('WebSocket connection established');
		};

		this.socket.onclose = () => {
			console.log('WebSocket connection closed');
		};

		this.socket.onerror = (error) => {
			console.error('WebSocket error:', error);
		};
	}

	disconnect() {
		if (this.socket) {
			this.socket.close();
			this.socket = null;
		}
	}
}
