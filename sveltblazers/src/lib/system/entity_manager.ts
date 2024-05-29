import type { Entity } from '$lib/entity/base';
import type { Player } from '$lib/entity/player';

export class EntityManager {
	private enemies: Entity[] = [];
	private players: Player[] = [];

	constructor() {}

	public addEnemy(enemy: Entity): void {
		this.enemies.push(enemy);
	}

	public addPlayer(player: Player): void {
		this.players.push(player);
	}

	public getPlayers(): Player[] {
		return this.players;
	}

	public getEnemies(): Entity[] {
		return this.enemies;
	}

	public cleanInactiveEntities(): void {
		this.enemies.filter((entity) => entity.active);
		this.players.filter((player) => player.active);
		this.allEntites().forEach((entity) => entity.cleanBullets());
	}

	public allEntites(): Entity[] {
		return [...this.enemies, ...this.players];
	}
}
