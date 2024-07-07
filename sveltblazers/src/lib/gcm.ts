export class GameConnection {
	private socket: WebSocket | null = null;
	private uuid: string;

	constructor(uuid: string) {
		this.uuid = uuid;
	}

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

	sendGameData(data: any) {
		if (this.socket && this.socket.readyState === WebSocket.OPEN) {
			this.socket.send(JSON.stringify(data));
		}
	}
}
