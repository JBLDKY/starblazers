import { Position } from "../types";

export abstract class Entity {
	position: Position;
	speed: number;

	constructor(position: Position, speed: number) {
		this.position = position;
		this.speed = speed;
	}

	abstract draw(ctx: CanvasRenderingContext2D): void;

	move(direction: Position): void {
		this.position.x += direction.x * this.speed;
	};

}

