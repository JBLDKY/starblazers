import { Alien } from '$lib/entity/alien';
import { Player } from '$lib/entity/player';
import { slowStraightShootingAlien } from '$lib/entity/slowStraightShootingAlien';
import type { Position } from '$lib/types';
import type { p5 } from 'p5-svelte';
import { EntityManager } from './entity_manager';

export class SpawnHandler {
	private p: p5;
	public spawn_counter: number = 0;
	private entityManager: EntityManager;

	constructor(p: p5, entityManager: EntityManager) {
		this.p = p;
		this.spawn_counter = 0;
		this.entityManager = entityManager;
	}

	public spawn_player(position: Position, speed: number, id: string): void {
		this.entityManager.addPlayer(new Player(this.p, position, speed, id));
	}

	public spawn(args: number[]): void {
		this.spawn_counter += 1;

		const typeId = args[0];
		const x = args[1] ?? 100;
		const y = args[2] ?? 100;
		const speed = args[3] ?? 1;
		// const color = args[5] ?? 'white';
		const position = { x, y };

		switch (typeId) {
			case 0:
				this.entityManager.addEnemy(
					new Alien(this.p, position, speed, this.spawn_counter.toString())
				);
				break;
			case 1:
				this.entityManager.addEnemy(
					new slowStraightShootingAlien(this.p, position, speed, this.spawn_counter.toString())
				);
				break;
			case 2:
				// this.alive.push(
				// 	new Bullet(position, speed, Boolean(dir), 'white', this.spawn_counter.toString())
				// );
				break;
		}
	}
}
