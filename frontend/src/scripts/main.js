"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
const game_1 = require("./game");
window.onload = () => {
    console.log("starting");
    const game = new game_1.SpaceInvadersGame('gameCanvas');
    game.start();
};
