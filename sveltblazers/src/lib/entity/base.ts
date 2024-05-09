import type p5 from 'p5';
import type { Position } from '../types';
import type { Bullet } from './bullet';
import type { SpawnHandler } from '$lib/system/spawn_handler';

const keyToVectorMap = {
	w: { x: 0, y: -1 },
	a: { x: -1, y: 0 },
	s: { x: 0, y: 1 },
	d: { x: 1, y: 0 }
};

export abstract class Entity {
	position: Position;
	speed: number;
	bullets: Bullet[] = [];
	destroy: boolean = false;
	id: string;

	constructor(position: Position, speed: number, id: string) {
		this.id = id;
		this.position = position;
		this.speed = speed;
		this.destroy = false;
	}

	abstract draw(p5: p5): void;
	abstract update(p5: p5, sh: SpawnHandler): void;

	getPosition(): Position {
		return { x: this.position.x, y: this.position.y };
	}

	drawDebug(p: p5) {
		p.fill(255, 0, 0);
		p.text(`ID: ${this.id}`, this.position.x + 100, this.position.y + 100);
		p.text(
			`Position: x: ${this.position.x}, y: ${this.position.y}`,
			this.position.x + 100,
			this.position.y + 130
		);
	}

	take_damage() {}

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
