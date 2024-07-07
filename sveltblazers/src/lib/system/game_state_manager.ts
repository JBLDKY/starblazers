// import type { WebSocketManager } from '$lib/websocketmanager';
//
// export class GameStateManager {
// 	private websocket: WebSocketManager;
//
// 	private getGameState: () => any;
// 	private setGameState: (state: any) => void;
//
// 	constructor(getGameState: () => any, setGameState: (state: any) => void) {
// 		this.getGameState = getGameState;
// 		this.setGameState = setGameState;
// 	}
//
// 	private handleGameStateUpdate(data: string): void {
// 		if (data == 'invalid json') {
// 			return;
// 		}
//
// 		this.setGameState(data);
// 	}
// }
