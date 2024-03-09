"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.SpaceInvadersGame = void 0;
const alien_1 = require("../entity/alien");
const player_1 = require("../entity/player");
const collisionManager_1 = require("./collisionManager");
const fpsmanager_1 = require("./fpsmanager");
/**
 * Represents the main game logic for a Space Invaders-like game.
 */
class SpaceInvadersGame {
    /**
     * Initializes the game with a given canvas.
     * @param {string} canvasId - The ID of the canvas element in the HTML document.
     */
    constructor(canvasId) {
        this.lastTime = 0;
        this.canvas = this.initCanvas(canvasId);
        this.ctx = this.canvas.getContext("2d");
        this.collisionManager = new collisionManager_1.CollisionManager();
        this.fpsManager = new fpsmanager_1.FPSManager(this.ctx);
        this.players = [];
        this.aliens = [];
    }
    /**
     * Starts the game loop. Sets up the player and initializes aliens.
     */
    start() {
        // Create player
        this.players.push(new player_1.Player({ x: this.canvas.width / 2, y: this.canvas.height - 30 }, 5));
        // Spawn some aliens
        this.initAliens();
        // Run gameloop through canvas
        requestAnimationFrame(() => this.gameLoop(this.lastTime));
    }
    /**
     * The main game loop. Updates game state and draws our background frames.
     */
    gameLoop(timestamp) {
        requestAnimationFrame((newTimestamp) => this.gameLoop(newTimestamp));
        if (this.fpsManager.shouldDraw(timestamp)) {
            this.update();
            this.draw();
        }
        this.fpsManager.update(timestamp);
    }
    /**
     * Updates the state of all game entities every loop/frame.
     */
    update() {
        const allBullets = this.getAllBullets();
        this.collisions(allBullets);
        // Bullets should probably take priority over other entities
        for (const bullet of allBullets) {
            bullet.update();
        }
        // Players
        for (const player of this.players) {
            player.update(this.ctx);
            // Explanation of Bullet Management:
            // Bullets are stored in each player's `bullets` attribute, which is an array of Bullet objects.
            // To stop rendering and processing a bullet (e.g., when it hits an alien or goes off-screen),
            // it must be removed from this array. Here, we loop through each player's bullets array and
            // filter out bullets marked for destruction (`bullet.destroy` is true).
            //
            // This approach is more efficient than using `getAllBullets()` for two reasons:
            // 1. Direct access: We can modify the `bullets` array directly within each player object.
            //    Using `getAllBullets()` would require an additional loop to link each bullet back to its respective player.
            // 2. Performance: It avoids the overhead of aggregating all bullets into a new array every frame.
            //
            // Future Consideration:
            // If bullet management becomes more complex or if there are performance issues with many bullets,
            // consider refactoring this to a more centralized bullet management system within the game class.
            player.bullets = player.bullets.filter((bullet) => !bullet.destroy);
        }
        // Update aliens, removing destroyed ones
        // This filtering is similar to bullet handling but simpler since aliens are directly managed by the game class.
        this.aliens = this.aliens.filter((alien) => !alien.destroy);
        // Only update aliens that are alive
        for (const alien of this.aliens) {
            alien.update(this.ctx);
        }
        // TODO: Implement player death
    }
    /**
     * Draws all game entities to the canvas.
     */
    draw() {
        // Clear canvas
        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
        // Draw players
        for (const player of this.players) {
            player.draw(this.ctx);
            // Draw each player's bullets
            // TODO: Figure out if this can be moved out of the player loop
            for (const bullet of player.bullets) {
                bullet.draw(this.ctx);
            }
        }
        // Draw aliens
        for (const alien of this.aliens) {
            alien.draw(this.ctx);
        }
        // Draw FPS
        this.fpsManager.draw();
    }
    /**
     * Checks and handles collisions between game entities.
     */
    collisions(allBullets) {
        for (const alien of this.aliens) {
            for (const bullet of allBullets) {
                // Check each alien against each bullet
                // TODO: Surely real gamedevs have tricks to reduce computation here
                if (this.collisionManager.checkCollision(alien, bullet)) {
                    alien.destroy = true;
                }
            }
        }
    }
    /**
     * Aggregates bullets from all players.
     * @returns {Bullet[]} An array of bullets from all players.
     */
    getAllBullets() {
        return this.players.flatMap((player) => player.bullets);
    }
    /**
     * Initializes aliens and positions them in a grid layout.
     */
    initAliens() {
        for (let i = 0; i < 5; i++) {
            // 5 rows
            for (let j = 0; j < 10; j++) {
                // 10 columns
                const x = 50 + j * 100;
                const y = 30 + i * 60;
                this.aliens.push(new alien_1.Alien({ x, y }, 0.1));
            }
        }
    }
    /**
     * Initializes the game canvas.
     * @param {string} canvasId - The ID of the canvas element.
     * @returns {HTMLCanvasElement} The initialized canvas element.
     */
    initCanvas(canvasId) {
        // TODO: create a new module that deals with more `meta` canvas stuff and move this there.
        const canvas = document.getElementById(canvasId);
        // TODO: figure out a computationally acceptable way to handle different sizes
        canvas.width = 1280;
        canvas.height = 800;
        return canvas;
    }
}
exports.SpaceInvadersGame = SpaceInvadersGame;
