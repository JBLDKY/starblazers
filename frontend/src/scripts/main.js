"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const game_1 = require("./game/game");
window.onload = () => {
    const game = new game_1.SpaceInvadersGame("gameCanvas");
    game.start();
};
