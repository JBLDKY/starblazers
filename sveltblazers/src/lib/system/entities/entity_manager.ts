import type { Alien } from '$lib/entity/alien';
import { Entity } from '$lib/entity/base';
import { Bullet } from '$lib/entity/bullet';
import { EntityIndex } from '$lib/entity/entity_index';
import type { Player } from '$lib/entity/player';
import { EntityEvent, type EntityEventHandler } from './entity_event_handler';

export class EntityManager implements EntityEventHandler {
	private entities: Map<number, Entity> = new Map();
	private freeIds: number[] = [...Array(1000).keys()].reverse();

	constructor() {}

	public addEntity(entity: Entity): void {
		console.log('Adding entity: ', entity);
		const entityId = this.nextId();
		entity.setId(entityId);
		entity.setEntityManager(this);
		this.entities.set(entityId, entity);
	}

	private freeId(id: number): void {
		this.freeIds.push(id);
	}

	private nextId(): number {
		const nextId = this.freeIds.pop();
		if (nextId === undefined) {
			throw new Error('Entity limit has been exceeded.');
		}
		return nextId;
	}

	public notify(entity: Entity, event: EntityEvent) {
		switch (event) {
			case EntityEvent.Fire:
				this.addEntity(entity.newBullet());
		}
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

	public getEntityByIndex(id: number): Entity | undefined {
		return this.entities.get(id);
	}

	public getBulletsByShooterId(shooterId: number): Bullet[] {
		const bullets: Bullet[] = [];
		for (const entity of this.entities.values()) {
			if (entity instanceof Bullet && entity.shooterId === shooterId) {
				bullets.push(entity as Bullet);
			}
		}

		return bullets;
	}

	public getPlayers(): Player[] {
		return this.getEntityByKind(EntityIndex.Player);
	}

	public getAliens(): Alien[] {
		return this.getEntityByKind(EntityIndex.Alien);
	}

	public getBullets(): Bullet[] {
		return this.getEntityByKind(EntityIndex.Bullet);
	}

	public cleanInactiveEntities(): void {
		for (const [id, entity] of this.entities) {
			if (!entity.active) {
				console.log('following entity is inactive: ');
				console.log(entity);
				console.log('deleting: ', id, ' ', entity);
				this.entities.delete(id);
				this.freeId(id);
			}
		}
	}

	public allEntites(): Entity[] {
		return [...this.entities.values()];
	}
}
