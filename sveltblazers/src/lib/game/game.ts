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
import type { Entity } from '$lib/entity/base';
import { DevConsole } from '$lib/dev_console';
import { SpawnHandler } from '$lib/system/spawn_handler';
import DebugManager from '$lib/system/debug_manager';
import { EntityManager } from '$lib/system/entity_manager';
import { InputHandler } from '$lib/system/input_handler';
import { MenuFactory, MenuIndex } from '$lib/entity/entity_index';

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
	private state: GameState = GameState.MENU;
	private currentMenu: BaseMenu | null;
	private devConsole: DevConsole = new DevConsole(this);
	public debugManager: DebugManager = new DebugManager();
	private entityManager: EntityManager = new EntityManager();
	public spawnHandler: SpawnHandler;
	private inputHandler: InputHandler;

	/**
	 * Initializes the game with a given p5 canvas.
	 */
	constructor(p: p5) {
		this.p = p;
		// Start the websocket
		this.websocket = new WebSocketManager();

		this.user = new User('');
		this.chatBox = new ChatBox(this.user, this.websocket);
		this.fpsManager = new FPSManager(this.p);
		this.spawnHandler = new SpawnHandler(this.p, this.entityManager);
		this.inputHandler = new InputHandler(this, this.devConsole);
		this.currentMenu = new MainMenu(this.p, this.inputHandler);
		this.collisionManager = new CollisionManager();
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

	startTypingMessage(): void {
		this.chatBox.startTypingMessage();
	}

	cancelMessage(): void {
		this.chatBox.cancelMessage();
	}

	sendMessage(): void {
		this.chatBox.sendMessage(this.devConsole);
	}

	getGameState(): GameState {
		return this.state;
	}

	setGameState(state: GameState): void {
		this.state = state;
	}

	getDevConsole(): GameState {
		return this.state;
	}

	handleInput(): void {
		this.inputHandler.handleInput();
	}

	setCurrentMenu(menuIndex: MenuIndex): void {
		this.currentMenu = new MenuFactory().newMenu(this.p, menuIndex, this.inputHandler);
	}

	public getCurrentPlayer(): Player {
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
					this.handleInput(); // TODO: why
					break;
				case GameState.MENU:
					if (this.currentMenu !== null && this.fpsManager.shouldProcessMenuInput(timestamp)) {
						this.currentMenu.loop();
					}
					break;
			}
		}
		this.chatBox.receiveMessage();
		this.fpsManager.update(timestamp);
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
}
