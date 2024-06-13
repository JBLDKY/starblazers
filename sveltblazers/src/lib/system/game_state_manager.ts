import type { WebSocketManager } from '$lib/websocketmanager';

export class GameStateManager {
	private websocket: WebSocketManager;

	private getGameState: () => any;
	private setGameState: (state: any) => void;

	constructor(
		websocket: WebSocketManager,
		getGameState: () => any,
		setGameState: (state: any) => void
	) {
		this.websocket = websocket;
		this.getGameState = getGameState;
		this.setGameState = setGameState;

		this.websocket.onMessage('gameStateUpdate', (data) => this.handleGameStateUpdate(data));

		this.websocket.onMessage('message', (data) => console.log(data));
	}

	private handleGameStateUpdate(data: string): void {
		console.log('setting gamestate: ', data);
	}

	public sendGameState(): void {
		this.websocket.sendMessage('gameStateUpdate', this.getGameState());
	}
}
