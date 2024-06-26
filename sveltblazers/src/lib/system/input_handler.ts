import type { DevConsole } from '$lib/dev_console';
import { MenuIndex } from '$lib/entity/entity_index';
import type { SpaceInvadersGame } from '$lib/game/game';
import { GameState } from '../../constants';

export class InputHandler {
	private game: SpaceInvadersGame;
	private keyPresses: { [key: string]: boolean } = {};
	private cachedKeyPresses: { [key: string]: boolean } = {};
	private devConsole: DevConsole;
	private lastDevCommandTime: number;

	constructor(game: SpaceInvadersGame, devConsole: DevConsole) {
		this.game = game;
		this.devConsole = devConsole;
		this.lastDevCommandTime = 0;
		document.addEventListener('keydown', this.handleKeyDown);
		document.addEventListener('keyup', this.handleKeyUp);
	}

	private handleKeyDown = (event: KeyboardEvent): void => {
		this.keyPresses[event.key] = true;

		if (this.game.getGameState() == GameState.MENU) {
			// Since the menu's run at lower tickrate, we must cache keypresses to be processed on the next tick
			this.cachedKeyPresses[event.key] = true; // remember to manually set to false
		}
	};

	private handleKeyUp = (event: KeyboardEvent): void => {
		this.keyPresses[event.key] = false;
	};

	private setLastDevCommandTime(timestamp: number) {
		this.lastDevCommandTime = timestamp;
	}

	// TODO: set to private
	public getLastDevCommandTime() {
		return this.lastDevCommandTime;
	}

	// Switch back to private after debugging
	public shouldHandleDevCommand(timestamp: number): boolean {
		return timestamp - this.getLastDevCommandTime() > 2000;
	}

	public handleInputWhileTyping() {
		if (this.keyPresses['Enter']) {
			this.game.sendMessage();
			return;
		}
	}
	public handleInput(timestamp: number) {
		// Start typing a message
		if (this.keyPresses['t'] || this.keyPresses['T']) {
			this.game.startTypingMessage();
			return;
		}

		if (this.keyPresses['1'] && this.shouldHandleDevCommand(timestamp)) {
			this.setLastDevCommandTime(timestamp);
			this.devConsole.handleCommand('debug');
			this.keyPresses['1'] = false;
			return;
		}

		if (this.keyPresses['2'] && this.shouldHandleDevCommand(timestamp)) {
			this.setLastDevCommandTime(timestamp);
			this.devConsole.handleCommand('spawn 0 600 100 0');
			return;
		}

		if ((this.keyPresses['p'] || this.keyPresses['P']) && this.shouldHandleDevCommand(timestamp)) {
			this.setLastDevCommandTime(timestamp);
			this.devConsole.handleCommand('spawn 1 540 100 0');
			return;
		}

		// Stop typing a message
		if (this.keyPresses['Escape']) {
			this.game.cancelMessage();

			switch (this.game.getGameState()) {
				case GameState.RUN:
					this.game.setGameState(GameState.MENU);
					this.game.setCurrentMenu(MenuIndex.Main);
					break;
				case GameState.PAUSE:
					this.game.setGameState(GameState.RUN);
					break;
			}

			return;
		}

		// Assuming this.keyPresses is an object containing the current state of WASD keys
		if (
			this.keyPresses['w'] ||
			this.keyPresses['a'] ||
			this.keyPresses['s'] ||
			this.keyPresses['d']
		) {
			const currentPlayer = this.game.getCurrentPlayer();
			if (currentPlayer != undefined) {
				currentPlayer.move(this.keyPresses);
				return;
			}

			console.error(
				'Tried to process movement input but `getCurrentPlayer()` returned `undefined`'
			);
		}
	}

	getCachedKeyPresses(): { [key: string]: boolean } {
		return this.cachedKeyPresses;
	}

	handleMenuResult(result: string) {
		switch (result) {
			case 'Multiplayer':
				this.game.setGameState(GameState.RUN);
				break;
			case 'Single player':
				this.game.setGameState(GameState.RUN);
				break;
			case 'Main menu':
				this.game.setGameState(GameState.MENU);
				this.game.setCurrentMenu(MenuIndex.Main);
				break;
			case 'Settings':
				this.game.setGameState(GameState.MENU);
				this.game.setCurrentMenu(MenuIndex.Settings);
				break;
		}
	}

	/**
	 * Destructor
	 */
	destroy() {
		document.removeEventListener('keydown', this.handleKeyDown);
		document.removeEventListener('keyup', this.handleKeyUp);
	}
}
