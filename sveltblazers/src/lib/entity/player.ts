import type p5 from 'p5';
import { Entity } from './base';
import { Key, type Rectangle } from '../types';
import { Bullet } from './bullet';
import { Colors } from '../assets/color';
import { EntityIndex } from './entity_index';
import type { Shooter } from './shooter';
import { EntityEvent } from '$lib/system/entities/entity_event_handler';

export class Player extends Entity implements Shooter {
	entityKind: EntityIndex = EntityIndex.Player;
	fireRate: number;
	cycles: number;
	uuid: string;

	height: number = 20;
	width: number = 20;

	constructor(p: p5, position: p5.Vector, uuid: string) {
		super(p, position);
		this.fireRate = 5;
		this.cycles = 0;
		this.uuid = uuid;
		this.velocity = this.p.createVector();
	}

	update() {
		const bullets = this.getBullets();
		if (bullets.length < this.maxBullets) {
			this.fire();
		}

		if (this.p.keyIsDown(Key.W)) {
			this.velocity.add(0, -1);
		}
		if (this.p.keyIsDown(Key.A)) {
			this.velocity.add(-1, 0);
		}
		if (this.p.keyIsDown(Key.S)) {
			this.velocity.add(0, 1);
		}
		if (this.p.keyIsDown(Key.D)) {
			this.velocity.add(1, 0);
		}

		this.position.add(this.velocity);
		this.velocity = this.p.createVector();

		this.cycles += 1;
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

		if (this.isDebugEnabled()) {
			this.drawDebug();
		}
	}

	shape(): Rectangle {
		return { pos: this.position, dimensions: { width: this.width, height: this.height } };
	}

	fire(): void {
		this.getEntityManager().notify(this, EntityEvent.Fire);
	}

	newBullet(): Bullet {
		return new Bullet(
			this.p,
			{ x: this.position.x, y: this.position.y },
			10,
			true,
			'pink',
			this.getId()
		);
	}
}
