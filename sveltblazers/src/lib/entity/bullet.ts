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
	id: string;

	constructor(p: p5, position: Position, speed: number, up: boolean, color: string, id: string) {
		super(p, position, speed, id);
		this.yVelocity = 1;
		this.direction = up ? 1 : -1;
		this.color = color;
		this.id = id;
	}

	rect(): Rectangle {
		return { pos: this.position, dimensions: { width: this.width, height: this.height } };
	}

	draw() {
		this.p.fill(this.color);
		this.p.rect(this.position.x, this.position.y, this.width, this.height);
	}

	update() {
		this.position.y += this.direction * this.speed * this.yVelocity;

		if (this.position.y < 0 || this.position.y > 800) {
			this.kill();
		}

		if (DebugManager.debugMode) {
			// this.drawDebug();
		}
	}
}
