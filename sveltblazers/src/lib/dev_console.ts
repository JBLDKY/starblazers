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
			console.warn('Dev console: Empty commands cannot be processed');
		}

		const args = command.split(' ');

		if (args === undefined) {
			console.error('Dev console: Args are undefined.');
			return;
		}

		const commandName = args.shift().toLowerCase();

		if (commandName === undefined) {
			console.error('Dev console: Args are undefined.');
			return;
		}

		const restArgs = args.map((arg) => Number.parseInt(arg));

		switch (commandName) {
			case 'enemies':
				console.log('debuggin enemies');
				this.game.getEnemies().forEach((entity) => entity.enableDebug());
				break;

			case 'printenemies':
				console.log('printing enemies');
				this.game.getEnemies().forEach((entity) => console.log(entity));
				break;

			case 'spawn':
				this.game.spawnHandler.spawn(restArgs);
				break;
			case 'set': // ID property value
				this.setEntityProperty(args[0], args[1], args[2]); // Example: set speed 20
				break;
			case 'debug':
				DebugManager.toggleDebugMode();
				break;
			case 'inspect':
				console.log('Dev console: ', this.game.getEntity(args[0]));
				break;
			default:
				console.log(`Dev console: Unknown command: ${command}`);
				break;
		}
	}

	setEntityProperty(id: string, property: string, value: string) {
		const entity = this.game.getEntity(id);
		if (entity === undefined) {
			console.error(`Dev console: No entity with id ${id} exists.`);
			return;
		}
		entity.setProperty(property, value);
	}
}
