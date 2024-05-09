import type { Entity } from '$lib/entity/base';
import type { Player } from '$lib/entity/player';
import type { slowStraightShootingAlien } from '$lib/entity/slowStraightShootingAlien';
import { Alien } from '../entity/alien';
import { Bullet } from '../entity/bullet';
import type { Position, Rectangle } from '../types';

// Type guard for Alien
function isAlien(entity: Entity): entity is Alien {
	return (entity as Alien).isAlien !== undefined;
}

// Type guard for SlowStraightShootingAlien
function isSlowStraightShootingAlien(entity: Entity): entity is Alien {
	return (entity as slowStraightShootingAlien).isSlowStraighShootingAlien !== undefined;
}

// Type guard for Player
function isPlayer(entity: Entity): entity is Player {
	return (entity as Player).isPlayer !== undefined;
}

export class CollisionManager {
	/**
	 * Checks collision between an alien and a bullet.
	 * @param {Alien} alien - The alien to check for collision.
	 * @param {Bullet} bullet - The bullet to check for collision.
	 * @returns {boolean} True if there is a collision, false otherwise.
	 */
	public checkCollision(entity: Entity, bullet: Bullet): boolean {
		if (isAlien(entity)) {
			return this.circleRectCollision(entity.position, 10, bullet.position, 5, 10);
		}

		if (isPlayer(entity)) {
			const playerRect = entity.rect();
			const bulletRect = bullet.rect();
			return this.rectRectCollision(playerRect, bulletRect);
		}

		if (isSlowStraightShootingAlien(entity)) {
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
