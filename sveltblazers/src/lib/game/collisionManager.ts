import { Alien } from '../entity/alien';
import { Bullet } from '../entity/bullet';
import type { Position } from '../types';

export class CollisionManager {
	/**
	 * Checks collision between an alien and a bullet.
	 * @param {Alien} alien - The alien to check for collision.
	 * @param {Bullet} bullet - The bullet to check for collision.
	 * @returns {boolean} True if there is a collision, false otherwise.
	 */
	public checkCollision(alien: Alien, bullet: Bullet): boolean {
		// TODO: Make function generic over Entity
		// Aliens are circles, bullets are rectangles
		return this.circleRectCollision(alien.position, 10, bullet.position, 5, 10);
		// Implement more collision checks
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

		const distanceX = circlePos.x - closestX;
		const distanceY = circlePos.y - closestY;

		const distanceSquared = distanceX * distanceX + distanceY * distanceY;
		return distanceSquared < circleRadius * circleRadius;
	}

	// Add more collision-related methods here
}
