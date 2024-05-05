import type p5 from 'p5';
import { Entity } from './base';
import type { Position } from '../types';
import { Colors } from '../assets/color';

export class Bullet extends Entity {
	yVelocity: number;
	destroy: boolean = false;

	constructor(position: Position, speed: number) {
		super(position, speed);
		this.yVelocity = 1;
	}

	draw(p5: p5) {
		p5.fillStyle = Colors.EFFECT;
		p5.rect(this.position.x, this.position.y, 5, 10);
	}

	update() {
		this.position.y -= 1 * this.speed * this.yVelocity;

		if (this.position.y < 0) {
			this.destroy = true;
		}
	}
}
