import type p5 from 'p5';
import { Entity } from './base';
import type { Position } from '../types';
import { Colors } from '../assets/color';
import DebugManager from '$lib/system/debug_manager';

export class Alien extends Entity {
	cycle: number;
	moveDown: boolean;
	xVelocity: number;
	id: string;
	radius: number = 10;
	isAlien: boolean = true;

	constructor(position: Position, speed: number, id: string) {
		super(position, speed, id);
		this.id = id;
		this.cycle = 0;
		this.moveDown = false;
		this.xVelocity = 30;
	}

	update(p5: p5) {
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
			(this.position.x >= p5.width && this.xVelocity > 0)
		) {
			this.moveDown = true;
		}
	}

	draw(p5: p5) {
		p5.fill(Colors.SECONDARY); // Fill first or else one will be the wrong color
		p5.circle(this.position.x, this.position.y, this.radius);

		if (DebugManager.debugMode) {
			this.drawDebug(p5);
		}
	}
}
