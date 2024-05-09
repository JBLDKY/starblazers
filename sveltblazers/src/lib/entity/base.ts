import type p5 from 'p5';
import type { Position, Shape } from '../types';
import type { Bullet } from './bullet';
import type { EntityIndex } from './entity_index';

const keyToVectorMap = {
	w: { x: 0, y: -1 },
	a: { x: -1, y: 0 },
	s: { x: 0, y: 1 },
	d: { x: 1, y: 0 }
};

export abstract class Entity {
	protected p: p5;
	protected bullets: Bullet[] = [];
	readonly id: string;
	abstract entityKind: EntityIndex;

	position: Position;
	speed: number;
	active: boolean = true;

	constructor(p: p5, position: Position, speed: number, id: string) {
		this.p = p;
		this.id = id;
		this.position = position;
		this.speed = speed;
		this.active = true;
	}

	abstract draw(): void;
	abstract update(): void;
	abstract shape(): Shape;

	getBullets(): Bullet[] {
		return this.bullets;
	}
	cleanBullets(): void {
		this.bullets = this.bullets.filter((bullet) => bullet.active);
	}

	getPosition(): Position {
		return { x: this.position.x, y: this.position.y };
	}

	drawDebug() {
		this.p.fill(255, 255, 255);

		const shape = this.shape();

		this.p.textSize(10);
		this.p.text(
			`ID: ${this.id}\nPosition: x: ${this.position.x}, y: ${this.position.y}`,
			this.position.x + shape.dimensions.width,
			this.position.y + shape.dimensions.height
		);
	}

	take_damage() {}

	kill() {
		this.active = false;
	}

	move(keyPresses: Record<string, boolean>): void {
		const movement = { x: 0, y: 0 };
		Object.entries(keyToVectorMap).forEach(([key, vector]) => {
			if (keyPresses[key]) {
				movement.x += vector.x;
				movement.y += vector.y;
			}
		});

		this.position.x += movement.x * this.speed;
		this.position.y += movement.y * this.speed;
	}
}
