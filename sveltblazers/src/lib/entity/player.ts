import type p5 from 'p5';
import { Entity } from './base';
import type { Position } from '../types';
import { Bullet } from './bullet';
import { Colors } from '../assets/color';
import { MAX_BULLETS } from '../../constants';

export class Player extends Entity {
	bullets: Bullet[];
	fireRate: number;
	cycles: number;
	uuid: string;

	constructor(position: Position, speed: number, uuid: string) {
		super(position, speed);
		this.bullets = [];
		this.fireRate = 5;
		this.cycles = 0;
		this.uuid = uuid;
	}
	update(p: p5) {
		if (this.bullets.length < MAX_BULLETS && this.cycles % this.fireRate == 0) {
			this.fire();
		}

		this.position.x = Math.max(0, Math.min(this.position.x, p.width));
		this.position.y = Math.max(0, Math.min(this.position.y, p.height));

		this.cycles += 1;
	}

	draw(p: p5): void {
		// Set the fill color
		p.fill(Colors.PRIMARY);

		// Draw the triangle
		p.triangle(
			this.position.x,
			this.position.y,
			this.position.x - 10,
			this.position.y + 20,
			this.position.x + 10,
			this.position.y + 20
		);
	}

	fire(): void {
		const x = this.position.x;
		const y = this.position.y;
		const position: Position = { x, y };
		this.bullets.push(new Bullet(position, 10));
	}
}
