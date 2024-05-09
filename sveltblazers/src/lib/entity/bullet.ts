import type p5 from 'p5';
import { Entity } from './base';
import type { Position, Rectangle } from '../types';
import DebugManager from '$lib/system/debug_manager';

export class Bullet extends Entity {
	yVelocity: number;
	destroy: boolean = false;
	width: number = 5;
	height: number = 10;
	direction: number;
	color: string;
	id: string;

	constructor(position: Position, speed: number, up: boolean, color: string, id: string) {
		super(position, speed, id);
		this.yVelocity = 1;
		this.direction = up ? 1 : -1;
		this.color = color;
		this.id = id;
	}

	rect(): Rectangle {
		return { pos: this.position, dimensions: { width: this.width, height: this.height } };
	}

	draw(p5: p5) {
		p5.fill(this.color);
		p5.rect(this.position.x, this.position.y, this.width, this.height);
	}

	update(p5: p5) {
		this.position.y += this.direction * this.speed * this.yVelocity;

		if (this.position.y < 0) {
			this.destroy = true;
		}

		if (this.position.y > 800) {
			this.destroy = true;
		}

		if (DebugManager.debugMode) {
			this.drawDebug(p5);
		}
	}
}
