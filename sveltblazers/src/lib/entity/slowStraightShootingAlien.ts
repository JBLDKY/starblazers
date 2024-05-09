import type p5 from 'p5';
import { Entity } from './base';
import type { Position } from '../types';
import { Bullet } from './bullet';
import DebugManager from '../system/debug_manager'; // Adjust path as necessary
import { EntityIndex } from './entity_index';

export class slowStraightShootingAlien extends Entity {
	entityKind: EntityIndex = EntityIndex.slowStraightShootingAlien;
	cycle: number = 0;
	radius: number = 10;
	moveDown: boolean;
	xVelocity: number;
	active: boolean;
	isSlowStraighShootingAlien: boolean = true;
	fireRate: number = 30;
	maxBullets: number = 10;
	image: p5.Image;
	damaged: p5.Image;
	idle: p5.Image;
	damage_frame: number | null;
	size: number;
	id: string;

	constructor(p: p5, position: Position, speed: number, id: string) {
		super(p, position, speed, id);

		this.idle = p.loadImage('/sprites/rock_boss_trimmed.png');
		this.damaged = p.loadImage('/sprites/rock_boss_trimmed_damaged.png');
		this.image = this.idle;

		this.cycle = 0;
		this.moveDown = false;
		this.xVelocity = 10;
		this.active = false;

		this.size = this.idle.width / 2;
		this.damage_frame = null;
		this.id = id;
	}

	update() {
		if (this.moveDown) {
			this.position.y += 30; // Move down
			this.xVelocity *= -1; // turn around (horizontally)
			this.moveDown = false;
		} else {
			this.position.x += this.speed * Number(this.xVelocity);
		}

		if (this.damage_frame != null && this.cycle - this.damage_frame == 20) {
			this.image = this.idle;
			this.damage_frame = null;
		}

		// Check if at the edge of the canvas and need to move down
		if (
			(this.position.x <= 0 && this.xVelocity < 0) ||
			(this.position.x + this.size * 2 >= this.p.width && this.xVelocity > 0)
		) {
			this.moveDown = true;
		}

		if (
			this.bullets.length < this.maxBullets &&
			this.cycle % this.fireRate == 0 &&
			Math.random() < 1
		) {
			this.fire();
		}

		this.cycle += 1;
	}

	draw() {
		this.p.image(this.image, this.position.x, this.position.y);
		this.bullets.forEach((bullet) => bullet.draw());

		if (DebugManager.debugMode) {
			this.drawDebug();
		}
	}

	take_damage() {
		console.log('taking damage');
		this.image = this.damaged;
		this.damage_frame = this.cycle;
	}

	fire(): void {
		this.bullets.push(
			new Bullet(this.p, { x: this.position.x, y: this.position.y }, 20, true, 'pink', 0)
		);
	}
}
