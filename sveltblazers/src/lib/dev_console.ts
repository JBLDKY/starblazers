import { SpaceInvadersGame } from './game/game';
import DebugManager from './system/debug_manager';

export class DevConsole {
	private game: SpaceInvadersGame;

	constructor(game: SpaceInvadersGame) {
		this.game = game;
	}

	handleCommand(command: string) {
		// Logic to handle different commands
		if (command.length == 0) {
			console.warn('Empty commands cannot be processed');
		}

		const args = command.split(' ');

		const commandName = args.shift().toLowerCase();

		const restArgs = args.map((arg) => Number.parseInt(arg));

		switch (commandName) {
			case 'spawn':
				this.game.spawnHandler.spawn(restArgs);
				break;
			case 'set':
				// this.game.setProperty(args[0], args[1]); // Example: set speed 20
				break;
			case 'debug':
				DebugManager.toggleDebugMode();
				break;
			case 'move':
				console.log('move command');
				break;
			default:
				console.log(`Unknown command: ${command}`);
		}
	}
}
