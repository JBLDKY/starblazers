import type { WebSocketManager } from '$lib/websocketmanager';

export class GameStateManager {
	private websocket: WebSocketManager;

	constructor(websocket: WebSocketManager) {
		this.websocket = websocket;

		this.websocket.onMessage('gameStateUpdate', (data) => this.handleGameStateUpdate(data));

		this.websocket.onMessage('message', (data) => console.log(data));
	}

	private handleGameStateUpdate(data: string): void {
		console.log('setting gamestate: ', data);
	}

	public sendGameState(): void {
		this.websocket.sendMessage('gameStateUpdate', 'player is moving');
	}
}
