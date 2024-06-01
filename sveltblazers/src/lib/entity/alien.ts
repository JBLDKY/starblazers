import type p5 from 'p5';
import { Entity } from './base';
import { Circle, type Position } from '../types';
import { Colors } from '../assets/color';
import DebugManager from '$lib/system/debug_manager';
import { EntityIndex } from './entity_index';
import { Bullet } from './bullet';
import type { Shooter } from './shooter';
import { EntityEvent } from '$lib/system/entities/entity_event_handler';

export class Alien extends Entity implements Shooter {
	entityKind: EntityIndex = EntityIndex.Alien;
	cycle: number;
	moveDown: boolean;
	xVelocity: number;
	radius: number = 10;
	isAlien: boolean = true;

	constructor(p: p5, position: Position, speed: number) {
		super(p, position, speed);
		this.cycle = 0;
		this.moveDown = false;
		this.xVelocity = 30;
	}

	shape(): Circle {
		return new Circle(this.position, this.radius);
	}

	newBullet(): Bullet {
		return new Bullet(this.p, this.position, 0, false, 'black', this.getId());
	}

	update() {
		if (this.moveDown) {
			this.position.y += 30; // Move down
			this.xVelocity *= -1; // turn around (horizontally)
			this.moveDown = false;
		} else {
			this.position.x += this.speed * Number(this.xVelocity);
		}

		// Check if at the edge of the canvas and need to move down
		if (
			(this.position.x <= 0 && this.xVelocity < 0) ||
			(this.position.x >= this.p.width && this.xVelocity > 0)
		) {
			this.moveDown = true;
		}
	}

	draw() {
		this.p.fill(Colors.SECONDARY); // Fill first or else one will be the wrong color
		this.p.circle(this.position.x, this.position.y, this.radius);

		if (DebugManager.debugMode) {
			this.drawDebug();
		}
	}

	public fire(): void {
		this.getEntityManager().notify(this, EntityEvent.Fire);
	}
}
