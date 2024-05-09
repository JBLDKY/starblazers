import { Alien } from '$lib/entity/alien';
import type { Entity } from '$lib/entity/base';
import { Bullet } from '$lib/entity/bullet';
import { Player } from '$lib/entity/player';
import { slowStraightShootingAlien } from '$lib/entity/slowStraightShootingAlien';
import type { Position } from '$lib/types';
import type { p5 } from 'p5-svelte';

export class SpawnHandler {
	public alive: Entity[];
	private p: p5;
	public spawn_counter: number = 0;
	private players: Player[];

	constructor(p: p5) {
		this.p = p;
		this.alive = [];
		this.spawn_counter = 0;
		this.players = [];
	}

	public getPlayers(): Player[] {
		return this.players;
	}

	public spawn_player(position: Position, speed: number, id: string): void {
		this.players.push(new Player(position, speed, id));
	}

	public spawn(args: number[]): void {
		this.spawn_counter += 1;

		const typeId = args[0];
		const x = args[1] ?? 100;
		const y = args[2] ?? 100;
		const speed = args[3] ?? 1;
		const dir = args[4] ?? true;
		// const color = args[5] ?? 'white';
		const position = { x, y };

		switch (typeId) {
			case 0:
				this.alive.push(new Alien(position, speed, this.spawn_counter.toString()));
				break;
			case 1:
				this.alive.push(
					new slowStraightShootingAlien(position, speed, this.p, this.spawn_counter.toString())
				);
				break;
			case 2:
				// this.alive.push(
				// 	new Bullet(position, speed, Boolean(dir), 'white', this.spawn_counter.toString())
				// );
				break;
		}
	}

	public cleanDeadEntities(): void {
		this.alive = this.alive.filter((entity) => !entity.destroy);
	}
}
