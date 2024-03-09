import { SpaceInvadersGame } from "./game/game";

window.onload = () => {
	const game = new SpaceInvadersGame("gameCanvas");
	game.start();
};
