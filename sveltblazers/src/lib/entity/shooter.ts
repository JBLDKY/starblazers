import { Bullet } from './bullet';

export interface Shooter {
	newBullet(): Bullet;
}
