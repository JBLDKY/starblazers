import type p5 from 'p5';
import { Entity } from './base';
import type { Position, Rectangle } from '../types';
import { Bullet } from './bullet';
import { Colors } from '../assets/color';
import DebugManager from '$lib/system/debug_manager';
import { EntityIndex } from './entity_index';
import type { Shooter } from './shooter';

export class Player extends Entity implements Shooter {
	entityKind: EntityIndex = EntityIndex.Player;
	fireRate: number;
	cycles: number;
	uuid: string;

	height: number = 20;
	width: number = 20;

	constructor(p: p5, position: Position, speed: number, uuid: string) {
		super(p, position, speed);
		this.fireRate = 5;
		this.cycles = 0;
		this.uuid = uuid;
	}

	update() {
		if (this.bullets.length < this.maxBullets && this.cycles % this.fireRate == 0) {
			this.fire();
		}

		this.position.x = Math.max(0, Math.min(this.position.x, this.p.width));
		this.position.y = Math.max(0, Math.min(this.position.y, this.p.height));

		this.cycles += 1;

		this.bullets.forEach((bullet) => bullet.update());
	}

	draw(): void {
		// Set the fill color
		this.p.fill(Colors.PRIMARY);

		// Draw the triangle
		this.p.triangle(
			this.position.x,
			this.position.y,
			this.position.x - 10,
			this.position.y + 20,
			this.position.x + 10,
			this.position.y + 20
		);

		this.bullets.forEach((bullet) => bullet.draw());

		if (DebugManager.debugMode) {
			this.drawDebug();
		}
	}

	shape(): Rectangle {
		return { pos: this.position, dimensions: { width: this.width, height: this.height } };
	}

	fire(): Bullet {
		return this.newBullet();
	}

	newBullet(): Bullet {
		return new Bullet(this.p, this.position, 1, true, 'pink', this.getId());
	}
}
