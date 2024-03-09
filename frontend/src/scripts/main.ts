import { SpaceInvadersGame } from "./game";

import { Alien } from "./entity/alien";
import { Player } from "./entity/player";
import { Bullet } from "./entity/bullet";

window.onload = () => {
	console.log("starting")
	const game = new SpaceInvadersGame('gameCanvas');
	game.start();
};
