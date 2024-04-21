import { Alien } from '../entity/alien';
import { Player } from '../entity/player';
import { Bullet } from '../entity/bullet';
import { CollisionManager } from './collisionManager';
import { FPSManager } from './fpsmanager';
import { ChatBox } from '../chat/chatbox';
import { User } from '../user/user';
import { WebSocketManager } from '../websocketmanager';

/**
 * Represents the main game logic for a Space Invaders-like game.
 */
export class SpaceInvadersGame {
	private canvas: HTMLCanvasElement;
	private ctx: CanvasRenderingContext2D;
	private collisionManager: CollisionManager;
	private websocket: WebSocketManager;
	private chatBox: ChatBox;
	private fpsManager: FPSManager;
	private players: Player[];
	private aliens: Alien[];
	private lastTime: number = 0;
	private user: User;
	private keyPresses: { [key: string]: boolean } = {};

	/**
	 * Initializes the game with a given canvas.
	 */
	constructor(canvas: HTMLCanvasElement) {
		this.canvas = canvas;
		this.ctx = this.canvas.getContext('2d')!;
		this.collisionManager = new CollisionManager();
		this.websocket = new WebSocketManager();
		// Start the websocket

		this.user = new User('');
		this.chatBox = new ChatBox(this.user, this.websocket);
		this.fpsManager = new FPSManager(this.ctx);
		this.players = [];
		this.aliens = [];

		document.addEventListener('keydown', this.handleKeyDown);
		document.addEventListener('keyup', this.handleKeyUp);
	}

	/**
	 * Starts the game loop. Sets up the player and initializes aliens.
	 */
	public start(): void {
		// Create player
		const player = new Player(
			{ x: this.canvas.width / 2, y: this.canvas.height - 30 },
			5,
			this.user.uuid
		);
		this.players.push(player);

		// Spawn some aliens
		this.initAliens();

		// Start websocket
		this.startWebsocket();

		// Run gameloop through canvas
		requestAnimationFrame(() => this.gameLoop(this.lastTime));
	}

	private getCurrentPlayer(): Player {
		return this.players.filter((player) => this.user.uuid == player.uuid)[0];
	}

	private startWebsocket() {
		this.websocket.connect();
	}

	/**
	 * The main game loop. Updates game state and draws our background frames.
	 */
	private gameLoop(timestamp: number): void {
		requestAnimationFrame((newTimestamp) => this.gameLoop(newTimestamp));

		if (this.fpsManager.shouldDraw(timestamp)) {
			this.update();
			this.draw();
		}

		this.fpsManager.update(timestamp);

		this.chatBox.receiveMessage();
	}

	private handleKeyDown = (event: KeyboardEvent): void => {
		this.keyPresses[event.key] = true;
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

		// Stop typing a message
		if (this.keyPresses['Escape']) {
			this.chatBox.cancelMessage();
			return;
		}

		// Send message
		if (this.keyPresses['Enter']) {
			console.log('enter pressed');
			this.chatBox.sendMessage();
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
	 * Updates the state of all game entities every loop/frame.
	 */
	private update(): void {
		this.handleInput();
		const allBullets = this.getAllBullets();

		this.collisions(allBullets);

		// Bullets should probably take priority over other entities
		for (const bullet of allBullets) {
			bullet.update();
		}

		// Players
		for (const player of this.players) {
			player.update(this.ctx);

			// Explanation of Bullet Management:
			// Bullets are stored in each player's `bullets` attribute, which is an array of Bullet objects.
			// To stop rendering and processing a bullet (e.g., when it hits an alien or goes off-screen),
			// it must be removed from this array. Here, we loop through each player's bullets array and
			// filter out bullets marked for destruction (`bullet.destroy` is true).
			//
			// This approach is more efficient than using `getAllBullets()` for two reasons:
			// 1. Direct access: We can modify the `bullets` array directly within each player object.
			//    Using `getAllBullets()` would require an additional loop to link each bullet back to its respective player.
			// 2. Performance: It avoids the overhead of aggregating all bullets into a new array every frame.
			//
			// Future Consideration:
			// If bullet management becomes more complex or if there are performance issues with many bullets,
			// consider refactoring this to a more centralized bullet management system within the game class.
			player.bullets = player.bullets.filter((bullet) => !bullet.destroy);
		}

		// Update aliens, removing destroyed ones
		// This filtering is similar to bullet handling but simpler since aliens are directly managed by the game class.
		this.aliens = this.aliens.filter((alien) => !alien.destroy);
		// Only update aliens that are alive
		for (const alien of this.aliens) {
			alien.update(this.ctx);
		}
		// TODO: Implement player death
	}

	/**
	 * Draws all game entities to the canvas.
	 */
	private draw(): void {
		// Clear canvas
		this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);

		// Draw players
		for (const player of this.players) {
			player.draw(this.ctx);

			// Draw each player's bullets
			// TODO: Figure out if this can be moved out of the player loop
			for (const bullet of player.bullets) {
				bullet.draw(this.ctx);
			}
		}

		// Draw aliens
		for (const alien of this.aliens) {
			alien.draw(this.ctx);
		}

		// Draw FPS
		this.fpsManager.draw();
	}

	/**
	 * Checks and handles collisions between game entities.
	 */
	private collisions(allBullets: Bullet[]): void {
		for (const alien of this.aliens) {
			for (const bullet of allBullets) {
				// Check each alien against each bullet
				// TODO: Surely real gamedevs have tricks to reduce computation here
				if (this.collisionManager.checkCollision(alien, bullet)) {
					alien.destroy = true;
				}
			}
		}
	}

	/**
	 * Aggregates bullets from all players.
	 * @returns {Bullet[]} An array of bullets from all players.
	 */
	private getAllBullets(): Bullet[] {
		return this.players.flatMap((player) => player.bullets);
	}

	/**
	 * Initializes aliens and positions them in a grid layout.
	 */
	private initAliens(): void {
		for (let i = 0; i < 5; i++) {
			// 5 rows
			for (let j = 0; j < 10; j++) {
				// 10 columns
				const x = 50 + j * 100;
				const y = 30 + i * 60;
				this.aliens.push(new Alien({ x, y }, 0.1));
			}
		}
	}

	/**
	 * Initializes the game canvas.
	 * @param {string} canvasId - The ID of the canvas element.
	 * @returns {HTMLCanvasElement} The initialized canvas element.
	 */
	// private initCanvas(canvasId: string): HTMLCanvasElement {
	// 	// TODO: create a new module that deals with more `meta` canvas stuff and move this there.
	// 	const canvas = document.getElementById(canvasId) as HTMLCanvasElement;
	// 	// TODO: figure out a computationally acceptable way to handle different sizes
	// 	canvas.width = 1280;
	// 	canvas.height = 800;
	// 	return canvas;
	// }

	/**
	 * Destructor
	 */
	destroy() {
		document.removeEventListener('keydown', this.handleKeyDown);
		document.removeEventListener('keyup', this.handleKeyUp);
	}
}
