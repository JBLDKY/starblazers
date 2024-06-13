import type p5 from 'p5';
import { Entity } from './base';
import type { Position, Rectangle } from '../types';
import DebugManager from '$lib/system/debug_manager';
import { EntityIndex } from './entity_index';

export class Bullet extends Entity {
	entityKind: EntityIndex = EntityIndex.Bullet;
	yVelocity: number;
	width: number = 5;
	height: number = 10;
	direction: number;
	color: string;
	shooterId: number;

	constructor(
		p: p5,
		position: Position,
		speed: number,
		up: boolean,
		color: string,
		shooterId: number
	) {
		super(p, position, speed);
		this.yVelocity = 1;
		this.direction = up ? -1 : 1;
		this.color = color;
		this.shooterId = shooterId;
	}

	shape(): Rectangle {
		return this.rect();
	}

	newBullet(): Bullet {
		return this;
	}

	rect(): Rectangle {
		return { pos: this.position, dimensions: { width: this.width, height: this.height } };
	}

	draw() {
		this.p.fill(this.color);
		this.p.rect(this.position.x, this.position.y, this.width, this.height);

		if (DebugManager.debugMode) {
			this.drawDebug();
		}
	}

	update() {
		this.position.y += this.direction * this.speed * this.yVelocity;

		if (this.position.y < 0 || this.position.y > 800) {
			this.kill();
		}
	}
}
