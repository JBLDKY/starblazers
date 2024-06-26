import type p5 from 'p5';
import type { Position, Shape } from '../types';
import type { Bullet } from './bullet';
import { EntityIndex } from './entity_index';
import type { EntityManager } from '$lib/system/entities/entity_manager';

const keyToVectorMap = {
	w: { x: 0, y: -1 },
	a: { x: -1, y: 0 },
	s: { x: 0, y: 1 },
	d: { x: 1, y: 0 }
};

export abstract class Entity {
	protected p: p5;
	abstract entityKind: EntityIndex;

	private id: number = -1; // The EntityManager should generate a unique ID for the entity
	private entityManager: EntityManager | null = null;
	private debug: boolean = false;

	maxBullets: number = 1;
	position: p5.Vector;
	velocity: p5.Vector;
	active: boolean = true;

	constructor(p: p5, position: p5.Vector) {
		this.p = p;
		this.position = position;
		this.velocity = this.p.createVector();
	}

	abstract draw(): void;
	abstract update(): void;
	abstract shape(): Shape;
	abstract newBullet(): Bullet;

	isDebugEnabled(): boolean {
		return this.debug;
	}

	toggleDebug() {
		this.debug = !this.debug;
	}

	enableDebug() {
		this.debug = true;
	}

	disableDebug() {
		this.debug = false;
	}

	protected getEntityManager(): EntityManager {
		if (!this.entityManager) {
			throw new Error('Mediator is not set');
		}
		return this.entityManager;
	}

	setEntityManager(entityManager: EntityManager): void {
		this.entityManager = entityManager;
	}

	setId(id: number): void {
		this.id = id;
	}

	getId(): number {
		if (this.id === -1) {
			throw new Error('ID is not set');
		}
		return this.id;
	}

	getPosition(): Position {
		return { x: this.position.x, y: this.position.y };
	}

	getBullets(): Bullet[] {
		if (this.entityManager == null) {
			throw new Error(`No entitymanager set on entity with ID: ${this.id}`);
		}

		return this.entityManager.getBulletsByShooterId(this.id);
	}

	drawDebug() {
		this.p.fill(255, 255, 255);

		const shape = this.shape();

		this.p.textSize(10);
		this.p.text(
			`id: ${this.id}\nposition: x: ${this.position.x}, y: ${this.position.y}\nspeed: ${this.velocity}`,
			this.position.x + shape.dimensions.width,
			this.position.y + shape.dimensions.height
		);
	}

	takeDamage() {}

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

		this.position.add(this.velocity);
	}

	setProperty(property: string, value: string) {
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

		this.velocity = value;
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
