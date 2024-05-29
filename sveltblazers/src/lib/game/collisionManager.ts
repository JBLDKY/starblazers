import type { Entity } from '$lib/entity/base';
import { EntityIndex } from '$lib/entity/entity_index';
import { Bullet } from '../entity/bullet';
import type { Position, Rectangle } from '../types';

export class CollisionManager {
	/**
	 * Checks collision between an alien and a bullet.
	 * @param {Entity} entity - The alien to check for collision.
	 * @param {Bullet} bullet - The bullet to check for collision.
	 * @returns {boolean} True if there is a collision, false otherwise.
	 */
	public checkCollision(entity: Entity, bullet: Bullet): boolean {
		switch (entity.entityKind) {
			case EntityIndex.Alien:
				return this.circleRectCollision(entity.position, 10, bullet.position, 5, 10);
			case EntityIndex.Player:
				return this.rectRectCollision(entity.shape(), bullet.rect());
			case EntityIndex.slowStraightShootingAlien:
				return this.circleRectCollision(entity.position, 136, bullet.position, 5, 10);
		}

		console.error('Unknown entity type: ', entity);
		return false;
	}

	/**
	 * Checks for a collision between a circle and a rectangle.
	 * @param {Position} circlePos - The position of the center of the circle.
	 * @param {number} circleRadius - The radius of the circle.
	 * @param {Position} rectPos - The position of the top-left corner of the rectangle.
	 * @param {number} rectWidth - The width of the rectangle.
	 * @param {number} rectHeight - The height of the rectangle.
	 * @returns {boolean} True if the circle and rectangle collide, false otherwise.
	 * TODO: Refactor all these geometric propertiens into the respective entity's class
	 */
	public circleRectCollision(
		circlePos: Position,
		circleRadius: number,
		rectPos: Position,
		rectWidth: number,
		rectHeight: number
	): boolean {
		const closestX = Math.max(rectPos.x, Math.min(circlePos.x, rectPos.x + rectWidth));
		const closestY = Math.max(rectPos.y, Math.min(circlePos.y, rectPos.y + rectHeight));

		const distanceX = circlePos.x + circleRadius - closestX;
		const distanceY = circlePos.y + circleRadius - closestY;

		const distanceSquared = distanceX * distanceX + distanceY * distanceY;
		return distanceSquared < circleRadius * circleRadius;
	}

	/**
	 * Determines if two rectangles are colliding.
	 * @param rect1 The first rectangle with properties { pos, width, height }
	 * @param rect2 The second rectangle with similar properties
	 * @returns true if the rectangles collide, false otherwise.
	 */
	public rectRectCollision(rect1: Rectangle, rect2: Rectangle): boolean {
		const xOverlap =
			rect1.pos.x + rect1.dimensions.width >= rect2.pos.x &&
			rect2.pos.x + rect2.dimensions.width >= rect1.pos.x;

		const yOverlap =
			rect1.pos.y + rect1.dimensions.height >= rect2.pos.y &&
			rect2.pos.y + rect2.dimensions.height >= rect1.pos.y;

		return xOverlap && yOverlap;
	}
}
