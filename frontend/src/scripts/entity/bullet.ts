import { Entity } from "./base";
import { Position } from "../types";

export class Bullet extends Entity {
	yVelocity: number;
	destroy: boolean = false;

	constructor(position: Position, speed: number) {
		super(position, speed);
		this.yVelocity = 1;
	}

	draw(ctx: CanvasRenderingContext2D) {
		ctx.fillStyle = "red";
		ctx.fillRect(this.position.x, this.position.y, 5, 10);
	};

	update(ctx: CanvasRenderingContext2D) {
		this.position.y -= 1 * this.speed * this.yVelocity

		if (this.position.y < 0) {
			this.destroy = true

		}
	}
}
