import type { Alien } from '$lib/entity/alien';
import type { Entity } from '$lib/entity/base';
import type { Bullet } from '$lib/entity/bullet';
import { EntityIndex } from '$lib/entity/entity_index';
import type { Player } from '$lib/entity/player';

export class EntityManager {
	private entities: Map<string, Entity> = new Map();

	constructor() {}

	public addEntity(entity: Entity): void {
		this.entities.set(entity.id, entity);
	}

	public getEntityByKind<K extends EntityIndex>(kind: K): Extract<Entity, { entityKind: K }>[] {
		const entities: Extract<Entity, { entityKind: K }>[] = [];
		for (const entity of this.entities.values()) {
			if (entity.entityKind === kind) {
				entities.push(entity as Extract<Entity, { entityKind: K }>);
			}
		}
		return entities;
	}

	public getPlayers(): Player[] {
		return this.getEntityByKind(EntityIndex.Player);
	}

	public getAliens(): Alien[] {
		return this.getEntityByKind(EntityIndex.Alien);
	}

	public getBulets(): Bullet[] {
		return this.getEntityByKind(EntityIndex.Bullet);
	}

	public cleanInactiveEntities(): void {
		for (const [id, entity] of this.entities) {
			if (!entity.active) {
				this.entities.delete(id);
			}
		}
	}

	public allEntites(): Entity[] {
		return [...this.entities.values()];
	}
}
