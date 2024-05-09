import { Player } from '../entity/player';
import { Bullet } from '../entity/bullet';
import { CollisionManager } from './collisionManager';
import { FPSManager } from './fpsmanager';
import { ChatBox } from '../chat/chatbox';
import { User } from '../user/user';
import { WebSocketManager } from '../websocketmanager';
import { Colors } from '$lib/assets/color';
import { GameState } from '../../constants';
import type p5 from 'p5';
import type { BaseMenu } from '$lib/menu/base';
import { MainMenu } from '$lib/menu/main';
import { SettingsMenu } from '$lib/menu/settings';
import type { Entity } from '$lib/entity/base';
import { DevConsole } from '$lib/dev_console';
import { SpawnHandler } from '$lib/system/spawn_handler';
import DebugManager from '$lib/system/debug_manager';
import { EntityManager } from '$lib/system/entity_manager';

const cartesian = (...a: any) => a.reduce((a, b) => a.flatMap((d) => b.map((e) => [d, e].flat())));
/**
 * Represents the main game logic for a Space Invaders-like game.
 */
export class SpaceInvadersGame {
	private p: p5;
	private collisionManager: CollisionManager;
	private websocket: WebSocketManager;
	private chatBox: ChatBox;
	private fpsManager: FPSManager;
	private lastTime: number = 0;
	private user: User;
	private keyPresses: { [key: string]: boolean } = {};
	private cachedKeyPresses: { [key: string]: boolean } = {};
	private state: GameState = GameState.MENU;
	private currentMenu: BaseMenu | null;
	private devConsole: DevConsole = new DevConsole(this);
	public debugManager: DebugManager = new DebugManager();
	public spawnHandler: SpawnHandler;
	private entityManager: EntityManager = new EntityManager();

	/**
	 * Initializes the game with a given p5 canvas.
	 */
	constructor(p: p5) {
		this.p = p;
		this.collisionManager = new CollisionManager();
		this.websocket = new WebSocketManager();
		// Start the websocket

		this.user = new User('');
		this.chatBox = new ChatBox(this.user, this.websocket);
		this.fpsManager = new FPSManager(this.p);

		this.currentMenu = new MainMenu(this.p);

		this.spawnHandler = new SpawnHandler(this.p, this.entityManager);

		document.addEventListener('keydown', this.handleKeyDown);
		document.addEventListener('keyup', this.handleKeyUp);
	}

	/**
	 * Starts the game loop. Sets up the player and initializes aliens.
	 */
	public start(): void {
		// Create player
		this.spawnHandler.spawn_player(
			{ x: this.p.width / 2, y: this.p.height - 30 },
			5,
			this.user.uuid
		);

		// Start websocket
		this.startWebsocket();

		// Run gameloop through p
		requestAnimationFrame(() => this.gameLoop(this.lastTime));
	}

	/**
	 * Updates the state of all game entities every loop/frame.
	 */
	public update(): void {
		this.entityManager.cleanInactiveEntities();
		this.collisions();

		this.entityManager.allEntites().forEach((entity) => entity.update());
	}

	/**
	 * Draws all game entities to the p.
	 */
	public draw(): void {
		// Clear p
		this.p.clear();
		this.p.background(Colors.BACKGROUND);

		this.entityManager.allEntites().forEach((entity) => entity.draw());

		// Draw FPS counter TODO: fix
		this.fpsManager.draw();
	}

	private getCurrentPlayer(): Player {
		return this.entityManager.getPlayers().filter((player) => this.user.uuid == player.uuid)[0];
	}

	private startWebsocket() {
		this.websocket.connect();
	}

	/**
	 * The main game loop. Updates game state and draws our background frames.
	 */
	private gameLoop(timestamp: number): void {
		requestAnimationFrame((newTimestamp) => this.gameLoop(newTimestamp));

		this.handleInput();
		if (this.fpsManager.shouldDraw(timestamp)) {
			switch (this.state) {
				case GameState.RUN:
					this.update();
					this.draw();
					break;
				case GameState.PAUSE:
					this.handleInput();
					break;
				case GameState.MENU:
					this.menuLoop(timestamp);
					break;
			}
		}

		this.chatBox.receiveMessage();
		this.fpsManager.update(timestamp);
	}

	private menuLoop(timestamp: number): void {
		if (this.currentMenu === null) {
			return;
		}

		let result;
		if (this.fpsManager.shouldProcessMenuInput(timestamp)) {
			result = this.currentMenu.handleInput(this.cachedKeyPresses);
		}
		if (result != '' && result != undefined) {
			this.handleMenuResult(result);
		}

		this.p.clear();
		this.currentMenu.display();
		this.p.rect(0, 30, 30, this.p.height);
		this.p.rect(this.p.width - 30, 30, 30, this.p.height);
	}

	private handleMenuResult(result: string) {
		switch (result) {
			case 'Multiplayer':
				this.state = GameState.RUN;
				break;
			case 'Single player':
				this.state = GameState.RUN;
				break;
			case 'Main menu':
				this.state = GameState.MENU;
				this.currentMenu = new MainMenu(this.p);
				break;
			case 'Settings':
				this.state = GameState.MENU;
				this.currentMenu = new SettingsMenu(this.p);
				break;
		}
	}

	private handleKeyDown = (event: KeyboardEvent): void => {
		this.keyPresses[event.key] = true;

		if (this.state == GameState.MENU) {
			// Since the menu's run at lower tickrate, we must cache keypresses to be processed on the next tick
			this.cachedKeyPresses[event.key] = true; // remember to manually set to false
		}
	};

	private handleKeyUp = (event: KeyboardEvent): void => {
		this.keyPresses[event.key] = false;
	};

	private handleInput() {
		// Start typing a message
		if (this.keyPresses['t'] || this.keyPresses['T']) {
			this.chatBox.startMessage();
			return;
		}

		if (this.keyPresses['p'] || this.keyPresses['P']) {
			this.devConsole.handleCommand('spawn 1 100 100 0');
			return;
		}

		// Stop typing a message
		if (this.keyPresses['Escape']) {
			this.chatBox.cancelMessage();

			switch (this.state) {
				case GameState.RUN:
					this.state = GameState.MENU;
					this.currentMenu = new MainMenu(this.p);
					break;
				case GameState.PAUSE:
					this.state = GameState.RUN;
					break;
			}

			return;
		}

		// Send message
		if (this.keyPresses['Enter']) {
			this.chatBox.sendMessage(this.devConsole);
			return;
		}

		// Assuming this.keyPresses is an object containing the current state of WASD keys
		if (
			this.keyPresses['w'] ||
			this.keyPresses['a'] ||
			this.keyPresses['s'] ||
			this.keyPresses['d']
		) {
			const currentPlayer = this.getCurrentPlayer();
			if (currentPlayer != undefined) {
				currentPlayer.move(this.keyPresses);
				return;
			}

			console.error(
				'Tried to process movement input but `getCurrentPlayer()` returned `undefined`'
			);
		}
	}

	/**
	 * Checks and handles collisions between game entities.
	 */
	private collisions(): void {
		// Check if any players are hit by creating a cartesian product of all players and enemy bullets
		cartesian(
			this.entityManager.getEnemies(),
			this.entityManager.getPlayers().flatMap((enemy) => enemy.getBullets())
		).forEach((pair: [Entity, Bullet]) => {
			if (this.collisionManager.checkCollision(pair[0], pair[1])) {
				pair[0].take_damage();
			}
		});

		// Check if any players are hit by creating a cartesian product of all players and enemy bullets
		cartesian(
			this.entityManager.getPlayers(),
			this.entityManager.getEnemies().flatMap((enemy) => enemy.getBullets())
		).forEach((pair: [Player, Bullet]) => {
			if (this.collisionManager.checkCollision(pair[0], pair[1])) {
				pair[0].active = false;
			}
		});
	}

	/**
	 * Destructor
	 */
	destroy() {
		document.removeEventListener('keydown', this.handleKeyDown);
		document.removeEventListener('keyup', this.handleKeyUp);
	}
}
