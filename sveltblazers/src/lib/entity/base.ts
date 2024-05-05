import type p5 from 'p5';
import type { Position } from '../types';

export abstract class Entity {
	position: Position;
	speed: number;

	constructor(position: Position, speed: number) {
		this.position = position;
		this.speed = speed;
	}

	abstract draw(p5: p5): void;

	move(keyPresses: Record<string, boolean>): void {
		const keyToVectorMap = {
			w: { x: 0, y: -1 },
			a: { x: -1, y: 0 },
			s: { x: 0, y: 1 },
			d: { x: 1, y: 0 }
		};

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
