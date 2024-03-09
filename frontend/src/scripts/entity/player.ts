import { Entity } from "./base";
import { Position } from "../types";
import { Bullet } from "./bullet";

export class Player extends Entity {
	bullets: Bullet[];
	fireRate: number;
	cycles: number;
	private keyPresses: { [key: string]: boolean } = {};

	constructor(position: Position, speed: number) {
		super(position, speed);
		this.bullets = [];
		this.fireRate = 5;
		this.cycles = 0;

		document.addEventListener('keydown', this.handleKeyDown);
		document.addEventListener('keyup', this.handleKeyUp);
	};

	private handleKeyDown = (event: KeyboardEvent): void => {
		this.keyPresses[event.key] = true;
		console.log(event);

	};

	private handleKeyUp = (event: KeyboardEvent): void => {
		this.keyPresses[event.key] = false;
	};

	update(ctx: CanvasRenderingContext2D) {
		if (this.keyPresses['w']) {
			this.position.y -= this.speed;
		}
		if (this.keyPresses['s']) {
			this.position.y += this.speed;
		}
		if (this.keyPresses['a']) {
			this.position.x -= this.speed;
		}
		if (this.keyPresses['d']) {
			this.position.x += this.speed;
		}

		if (this.bullets.length < 100 && this.cycles % this.fireRate == 0) {
			this.fire()
		}

		this.position.x = Math.max(0, Math.min(this.position.x, ctx.canvas.width));
		this.position.y = Math.max(0, Math.min(this.position.y, ctx.canvas.height));

		this.cycles += 1;
	};

	draw(ctx: CanvasRenderingContext2D): void {
		ctx.beginPath();
		ctx.moveTo(this.position.x, this.position.y);
		ctx.lineTo(this.position.x - 10, this.position.y + 20);
		ctx.lineTo(this.position.x + 10, this.position.y + 20);
		ctx.closePath();
		ctx.fillStyle = "pink";
		ctx.fill();
	};

	fire(): void {
		let x = this.position.x
		let y = this.position.y
		let position: Position = { x, y };
		this.bullets.push(new Bullet(position, 10));
	};


	destroy() {
		document.removeEventListener('keydown', this.handleKeyDown);
		document.removeEventListener('keyup', this.handleKeyUp);
	}
};
