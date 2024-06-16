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
import { SpawnHandler } from '$lib/system/entities/spawn_handler';
import DebugManager from '$lib/system/debug_manager';
import { EntityManager } from '$lib/system/entities/entity_manager';
import { InputHandler } from '$lib/system/input_handler';
import { EntityIndex, MenuFactory, MenuIndex } from '$lib/entity/entity_index';
import { GameStateManager } from '$lib/system/game_state_manager';

// eslint-disable-next-line @typescript-eslint/no-explicit-any
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
	private user: User;
	private state: GameState = GameState.MENU;
	private currentMenu: BaseMenu | null;
	private devConsole: DevConsole = new DevConsole(this);
	private entityManager: EntityManager = new EntityManager();
	private inputHandler: InputHandler;

	public debugManager: DebugManager = new DebugManager();
	public spawnHandler: SpawnHandler;
	public gameStateManager: GameStateManager;

	/**
	 * Initializes the game with a given p5 canvas.
	 */
	constructor(p: p5, player_id: string) {
		this.p = p;

		// These methods are passed to gameStateManager who calls them
		// to get and set gamestate. They must be bound to this class
		// because methods called inside of these might not exist on gamestatemanager
		this.getGameStateData = this.getGameStateData.bind(this);
		this.setGameStateData = this.setGameStateData.bind(this);
		this.websocket = new WebSocketManager(this.setGameStateData);

		this.user = new User('username', player_id);
		this.chatBox = new ChatBox(this.user, this.websocket);
		this.fpsManager = new FPSManager();
		this.spawnHandler = new SpawnHandler(this.p, this.entityManager);
		this.inputHandler = new InputHandler(this, this.devConsole);
		this.currentMenu = new MainMenu(this.p, this.inputHandler);
		this.collisionManager = new CollisionManager();

		this.gameStateManager = new GameStateManager(
			this.websocket,
			this.getGameStateData,
			this.setGameStateData
		);
	}

	/**
	 * Starts the game loop. Sets up the player and initializes aliens.
	 */
	public start(): void {
		// Create player
		this.spawnHandler.spawn_player(this.p.createVector(640, 730), this.user.uuid);

		// Start websocket
		this.startWebsocket();

		// Run gameloop through p
		requestAnimationFrame(() => this.gameLoop(0));
	}

	/**
	 * Updates the state of all game entities every loop/frame.
	 */
	public update(timestamp: number): void {
		if (!this.isTypingInChat()) {
			this.handleInput(timestamp);
		} else {
			this.handleInputWhileTyping();
		}

		this.entityManager.cleanInactiveEntities();
		this.collisions();

		this.entityManager.allEntities().forEach((entity) => entity.update());
	}

	/**
	 * Draws all game entities to the p.
	 */
	public draw(): void {
		this.clearCanvas();
		this.drawBackground();
		this.drawEntities();
		this.debugInfo();
	}

	private clearCanvas(): void {
		this.p.clear();
	}

	private drawBackground(): void {
		this.p.background(Colors.BACKGROUND);
	}

	private drawEntities(): void {
		this.entityManager.allEntities().forEach((entity: Entity) => {
			entity.draw();
			if (entity.isDebugEnabled()) {
				entity.drawDebug();
			}
		});
	}

	private debugInfo(): void {
		const debugMessages = [
			'FPS: ' + Math.round(this.p.frameRate()),
			'Chatting: ' + this.isTypingInChat(),
			'Dev command: ' + this.inputHandler.shouldHandleDevCommand(this.fpsManager.getInGameTime()),
			'Last dev cmd time: ' + this.inputHandler.getLastDevCommandTime(),
			'Debug: ' + DebugManager.debugMode,
			'Frame: ' + this.fpsManager.getFrameCount(),
			'IGT: ' + Math.trunc(this.fpsManager.getInGameTime() / 1000),
			'Entity count: ' + this.entityManager.allEntities().length,
			'Player id: ' + this.user.uuid
		];

		this.displayDebugInfo(debugMessages);
	}

	private displayDebugInfo(messages: string[]): void {
		let yPos = 400;
		this.p.textSize(10);
		this.p.fill('white');
		for (const message of messages) {
			this.p.text(message, 50, yPos);
			yPos += 10;
		}
	}

	setMessage(value: string): void {
		this.chatBox.setMessage(value);
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

	getEntity(id: number): Entity | undefined {
		return this.entityManager.allEntities().find((entity) => entity.getId() == id);
	}

	getGameState(): GameState {
		return this.state;
	}

	setGameState(state: GameState): void {
		this.state = state;
	}

	handleInput(timestamp: number): void {
		this.inputHandler.handleInput(timestamp);
	}

	handleInputWhileTyping(): void {
		this.inputHandler.handleInputWhileTyping();
	}

	setCurrentMenu(menuIndex: MenuIndex): void {
		this.currentMenu = new MenuFactory().newMenu(this.p, menuIndex, this.inputHandler);
	}

	isTypingInChat(): boolean {
		return this.chatBox.isTypingInChat();
	}

	public getCurrentPlayer(): Player {
		return this.entityManager
			.getEntityByKind(EntityIndex.Player)
			.filter((player: Player) => this.user.uuid == player.uuid)[0];
	}

	public getPlayerByUuid(uuid: string): Player {
		return this.entityManager
			.getEntityByKind(EntityIndex.Player)
			.filter((player: Player) => uuid == player.uuid)[0];
	}

	private startWebsocket() {
		this.websocket.connect();
	}

	/**
	 * The main game loop. Updates game state and draws our background frames.
	 */
	private gameLoop(timestamp: number): void {
		requestAnimationFrame((newTimestamp) => {
			this.gameLoop(newTimestamp);
		});

		if (this.fpsManager.shouldDraw(timestamp)) {
			switch (this.state) {
				case GameState.RUN:
					this.update(timestamp);
					this.draw();
					this.gameStateManager.sendGameState(); // Send game state update
					break;
				case GameState.PAUSE:
					this.handleInput(timestamp); // TODO: why
					break;
				case GameState.MENU:
					if (this.currentMenu !== null && this.fpsManager.shouldProcessMenuInput(timestamp)) {
						this.p.fill('deeppink'); // This fixes the bug where subsequent menus are white
						this.currentMenu.loop();
					}
					break;
			}
		}
		// this.chatBox.receiveMessage();
		this.fpsManager.update(timestamp);
	}

	/**
	 * Checks and handles collisions between game entities.
	 */
	private collisions(): void {
		// Check if any players are hit by creating a cartesian product of all players and enemy bullets
		cartesian(
			this.entityManager.getAliens(),
			this.entityManager.getPlayers().flatMap((enemy) => enemy.getBullets())
		).forEach((pair: [Entity, Bullet]) => {
			if (this.collisionManager.checkCollision(pair[0], pair[1])) {
				pair[0].takeDamage();
			}
		});

		// Check if any players are hit by creating a cartesian product of all players and enemy bullets
		cartesian(
			this.entityManager.getPlayers(),
			this.entityManager.getAliens().flatMap((enemy) => enemy.getBullets())
		).forEach((pair: [Player, Bullet]) => {
			if (this.collisionManager.checkCollision(pair[0], pair[1])) {
				pair[0].active = false;
			}
		});
	}

	getEnemies(): Entity[] {
		return this.entityManager.getEntityByKind(EntityIndex.slowStraightShootingAlien);
	}

	getGameStateData(): string {
		const player = this.getCurrentPlayer();

		return JSON.stringify({
			type: 'GameState',
			position_x: player.getPosition().x,
			position_y: player.getPosition().y,
			player_id: this.user.uuid,
			timestamp: new Date()
		});
	}

	setGameStateData(gamestate: string): void {
		const friend = this.getPlayerByUuid(gamestate.player_id);

		if (friend === undefined) {
			this.spawnHandler.spawn_player(this.p.createVector(640, 730), gamestate.player_id);
			return;
		}

		friend.setXPos(gamestate.position_x);
		friend.setYPos(gamestate.position_y);
	}

	createLobby(): void {
		console.log('creating lobby');
	}
}
