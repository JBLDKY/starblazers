import type { Entity } from '$lib/entity/base';

export enum EntityEvent {
	Fire
}

export interface EntityEventHandler {
	addEntity(entity: Entity): void;
	notify(entity: Entity, event: EntityEvent): void;
}
