import p5 from 'p5-svelte';
import { Entity } from './base';
import type { Position } from '../types';
import { Colors } from '../assets/color';

export class Alien extends Entity {
	cycle: number;
	moveDown: boolean;
	xVelocity: number;
	destroy: boolean;

	constructor(position: Position, speed: number) {
		super(position, speed);
		this.cycle = 0;
		this.moveDown = false;
		this.xVelocity = 30;
		this.destroy = false;
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
			(this.position.x >= p5.canvas.width && this.xVelocity > 0)
		) {
			this.moveDown = true;
		}
	}

	draw(p5: p5) {
		p5.fill(Colors.SECONDARY); // Fill first or else one will be the wrong color
		p5.circle(this.position.x, this.position.y, 10, 0, 2 * Math.PI);
	}
}
