import type p5 from 'p5';
import { Entity } from './base';
import { Circle } from '../types';
import type { Position } from '../types';
import { EntityIndex } from './entity_index';
import type { Shooter } from './shooter';
import { EntityEvent } from '$lib/system/entities/entity_event_handler';
import { Bullet } from './bullet';

export class slowStraightShootingAlien extends Entity implements Shooter {
	entityKind: EntityIndex = EntityIndex.slowStraightShootingAlien;
	cycle: number = 0;
	radius: number;
	moveDown: boolean;
	xVelocity: number;
	fireRate: number = 30;
	maxBullets: number = 10;
	image: p5.Image;
	damaged: p5.Image;
	idle: p5.Image;
	damage_frame: number | null;
	size: number;

	constructor(p: p5, position: Position, speed: number) {
		super(p, position, speed);

		this.idle = p.loadImage('/sprites/rock_boss_trimmed.png');
		this.damaged = p.loadImage('/sprites/rock_boss_trimmed_damaged.png');
		this.image = this.idle;

		this.cycle = 0;
		this.moveDown = false;
		this.xVelocity = 10;

		this.radius = this.image.width / 2;
		this.size = this.image.width / 2;
		this.damage_frame = null;
	}

	shape(): Circle {
		return new Circle(this.position, this.radius);
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

		if (this.cycle % this.fireRate == 0 && Math.random() < 1) {
			this.fire();
		}

		this.cycle += 1;
	}

	draw() {
		this.p.image(this.image, this.position.x, this.position.y);

		if (this.isDebugEnabled()) {
			this.drawDebug();
		}
	}

	takeDamage() {
		this.image = this.damaged;
		this.damage_frame = this.cycle;
	}

	public fire(): void {
		this.getEntityManager().notify(this, EntityEvent.Fire);
	}

	newBullet(): Bullet {
		return new Bullet(
			this.p,
			{ x: this.position.x, y: this.position.y },
			100,
			false,
			'orange',
			this.getId()
		);
	}
}
