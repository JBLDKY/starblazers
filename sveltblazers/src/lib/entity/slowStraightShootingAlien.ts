import type p5 from 'p5';
import { Entity } from './base';
import type { Position } from '../types';
import { Bullet } from './bullet';
import DebugManager from '../system/debug_manager'; // Adjust path as necessary
import type { SpawnHandler } from '$lib/system/spawn_handler';

export class slowStraightShootingAlien extends Entity {
	cycle: number = 0;
	radius: number = 10;
	moveDown: boolean;
	xVelocity: number;
	destroy: boolean;
	isSlowStraighShootingAlien: boolean = true;
	fireRate: number = 30;
	maxBullets: number = 10;
	image: p5.Image;
	damaged: p5.Image;
	idle: p5.Image;
	damage_frame: number | null;
	size: number;
	id: string;

	constructor(position: Position, speed: number, p: p5, id: string) {
		super(position, speed, id);

		this.idle = p.loadImage('/sprites/rock_boss_trimmed.png');
		this.damaged = p.loadImage('/sprites/rock_boss_trimmed_damaged.png');
		this.image = this.idle;

		this.cycle = 0;
		this.moveDown = false;
		this.xVelocity = 10;
		this.destroy = false;

		this.size = this.idle.width / 2;
		this.damage_frame = null;
		this.id = id;
	}

	update(p5: p5, sh: SpawnHandler) {
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
			(this.position.x + this.size * 2 >= p5.width && this.xVelocity > 0)
		) {
			this.moveDown = true;
		}

		if (
			this.bullets.length < this.maxBullets &&
			this.cycle % this.fireRate == 0 &&
			Math.random() < 1
		) {
			this.fire(sh);
		}

		this.cycle += 1;
	}

	draw(p5: p5) {
		p5.image(this.image, this.position.x, this.position.y);
		// p5.fill('orange'); // Fill first or else one will be the wrong color
		// p5.circle(this.position.x, this.position.y, this.radius);
		//
		if (DebugManager.debugMode) {
			this.drawDebug(p5);
		}
	}

	take_damage() {
		console.log('taking damage');
		this.image = this.damaged;
		this.damage_frame = this.cycle;
	}

	fire(sh: SpawnHandler): void {
		const x = this.position.x + this.image.width / 2;
		const y = this.position.y + this.image.height / 2;
		sh.spawn([2, x, y, 3, 1, 'orange']);
	}
}
