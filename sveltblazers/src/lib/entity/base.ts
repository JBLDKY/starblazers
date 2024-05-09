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

	maxBullets: number = 1;
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
			`id: ${this.id}\nposition: x: ${this.position.x}, y: ${this.position.y}\nbullets: ${this.bullets.length}\nspeed: ${this.speed}`,
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

	setProperty(property: string, value: any) {
		if (property in ['x', 'y']) {
			property = 'position' + property;
		}

		switch (property) {
			case 'speed':
				this.setSpeed(value);
				break;
			case 'x':
				this.setXPos(value);
				break;
			case 'y':
				this.setYPos(value);
				break;
			case 'maxBullets':
				this.setMaxBullets(value);
				break;
			default:
				console.error(`Property does not exist ${property}.`);
		}
	}

	setSpeed(value: string | number) {
		if (typeof value === 'string') {
			value = Number.parseInt(value);
		}

		this.speed = value;
	}

	setXPos(value: string | number) {
		if (typeof value === 'string') {
			value = Number.parseInt(value);
		}

		this.position.x = value;
	}

	setYPos(value: string | number) {
		if (typeof value === 'string') {
			value = Number.parseInt(value);
		}

		this.position.y = value;
	}

	setMaxBullets(value: string | number) {
		if (typeof value === 'string') {
			value = Number.parseInt(value);
		}

		this.maxBullets = value;
	}
}
