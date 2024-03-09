import { Alien } from "./entity/alien";
import { Player } from "./entity/player";
import { Bullet } from "./entity/bullet";
import { Position } from "./types";

export class SpaceInvadersGame {
	private canvas: HTMLCanvasElement;
	private ctx: CanvasRenderingContext2D;
	private players: Player[];
	private aliens: Alien[];
	private bullets: Bullet[];

	constructor(canvasId: string) {
		this.canvas = this.initCanvas(canvasId);
		this.ctx = this.canvas.getContext('2d')!;
		this.players = [];
		this.aliens = [];
		this.bullets = [];
	}

	public start(): void {
		if (!this.ctx) {
			console.error("No canvas to run game in"); // Guard 
			return
		};

		// Create player
		this.players.push(new Player({ x: this.canvas.width / 2, y: this.canvas.height - 30 }, 5));

		this.initAliens();

		requestAnimationFrame(() => this.gameLoop());
	}

	private gameLoop(): void {
		this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height); // Clear screen
		this.update(); // Process logic
		this.draw(); // Render everything
		requestAnimationFrame(() => this.gameLoop()); // Advance frame
	}

	private initAlien(): void {
		let x = 50;
		let y = 50;

		this.aliens.push(new Alien({ x, y }, 100));
	}

	private initAliens(): void {
		for (let i = 0; i < 5; i++) { // 5 rows
			for (let j = 0; j < 10; j++) { // 10 columns
				const x = 50 + j * 100;
				const y = 30 + i * 60;
				this.aliens.push(new Alien({ x, y }, 0.1));
			}
		}
	}

	private initCanvas(canvasId: string): HTMLCanvasElement {
		const canvas = document.getElementById(canvasId) as HTMLCanvasElement;
		canvas.width = 1280;
		canvas.height = 800;
		return canvas
	}

	private getAllBullets(): Bullet[] {
		let bullets: Bullet[] = [];

		for (const player of this.players) {
			bullets.push(...player.bullets)
		}

		return bullets
	}

	private collisions(): void {
		const allBullets = this.getAllBullets();
		for (const alien of this.aliens) {
			for (const bullet of allBullets) {
				if (this.checkCollision(alien, bullet)) {
					alien.destroy = true;

				}
			}
		}


	}

	public getCanvas(): HTMLCanvasElement {
		return this.canvas
	}

	private checkCollision(alien: Alien, bullet: Bullet): boolean {
		// Handle different shapes
		return this.circleRectCollision(alien.position, 10, bullet.position, 5, 10);
	}

	private circleRectCollision(circlePos: Position, circleRadius: number, rectPos: Position, rectWidth: number, rectHeight: number): boolean {
		const closestX = Math.max(rectPos.x, Math.min(circlePos.x, rectPos.x + rectWidth));
		const closestY = Math.max(rectPos.y, Math.min(circlePos.y, rectPos.y + rectHeight));

		const distanceX = circlePos.x - closestX;
		const distanceY = circlePos.y - closestY;

		const distanceSquared = (distanceX * distanceX) + (distanceY * distanceY);
		return distanceSquared < (circleRadius * circleRadius);
	}

	private update(): void {
		this.collisions();

		for (const player of this.players) {
			player.update(this.ctx);

			for (const bullet of player.bullets) {
				bullet.update(this.ctx);
			}

			player.bullets = player.bullets.filter(bullet => !bullet.destroy);

		}

		this.aliens = this.aliens.filter(alien => !alien.destroy)
		for (const alien of this.aliens) {
			alien.update(this.ctx);
		}
	}

	private draw(): void {
		// Clear canvas
		this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);

		// Draw player
		for (const player of this.players) {
			player.draw(this.ctx);

			// Draw bullets
			for (const bullet of player.bullets) {
				bullet.draw(this.ctx);
			}
		}

		// Draw aliens
		for (const alien of this.aliens) {
			alien.draw(this.ctx);
		}

	}
}
