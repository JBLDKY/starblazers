import type p5 from 'p5';
import { Entity } from './base';
import type { Position, Rectangle } from '../types';
import { Bullet } from './bullet';
import { Colors } from '../assets/color';
import { MAX_BULLETS } from '../../constants';
import type { SpawnHandler } from '$lib/system/spawn_handler';
import DebugManager from '$lib/system/debug_manager';

export class Player extends Entity {
	fireRate: number;
	cycles: number;
	uuid: string;
	height: number = 20;
	width: number = 20;
	isPlayer: boolean = true;

	constructor(position: Position, speed: number, uuid: string) {
		super(position, speed, uuid);
		this.fireRate = 5;
		this.cycles = 0;
		this.uuid = uuid;
	}

	rect(): Rectangle {
		return { pos: this.position, dimensions: { width: this.width, height: this.height } };
	}

	update(p: p5, sh: SpawnHandler) {
		if (this.bullets.length < MAX_BULLETS && this.cycles % this.fireRate == 0) {
			this.fire(sh);
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

		if (DebugManager.debugMode) {
			this.drawDebug(p);
		}
	}

	fire(sh: SpawnHandler): void {
		// const x = this.position.x;
		// const y = this.position.y;
		// sh.spawn([2, x, y, 3, 0, 'orange']);
		this.bullets.push(
			new Bullet({ x: this.position.x, y: this.position.y }, 20, false, 'white', 0)
		);
	}
}
