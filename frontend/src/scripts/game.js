"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.SpaceInvadersGame = void 0;
const alien_1 = require("./entity/alien");
const player_1 = require("./entity/player");
class SpaceInvadersGame {
    constructor(canvasId) {
        this.canvas = this.initCanvas(canvasId);
        this.ctx = this.canvas.getContext('2d');
        this.players = [];
        this.aliens = [];
        this.bullets = [];
    }
    start() {
        if (!this.ctx) {
            console.error("No canvas to run game in"); // Guard 
            return;
        }
        ;
        // Create player
        this.players.push(new player_1.Player({ x: this.canvas.width / 2, y: this.canvas.height - 30 }, 5));
        this.initAliens();
        requestAnimationFrame(() => this.gameLoop());
    }
    gameLoop() {
        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height); // Clear screen
        this.update(); // Process logic
        this.draw(); // Render everything
        requestAnimationFrame(() => this.gameLoop()); // Advance frame
    }
    initAlien() {
        let x = 50;
        let y = 50;
        this.aliens.push(new alien_1.Alien({ x, y }, 100));
    }
    initAliens() {
        for (let i = 0; i < 5; i++) { // 5 rows
            for (let j = 0; j < 10; j++) { // 10 columns
                const x = 50 + j * 100;
                const y = 30 + i * 60;
                this.aliens.push(new alien_1.Alien({ x, y }, 0.1));
            }
        }
    }
    initCanvas(canvasId) {
        const canvas = document.getElementById(canvasId);
        canvas.width = 1280;
        canvas.height = 800;
        return canvas;
    }
    getAllBullets() {
        let bullets = [];
        for (const player of this.players) {
            bullets.push(...player.bullets);
        }
        return bullets;
    }
    collisions() {
        const allBullets = this.getAllBullets();
        for (const alien of this.aliens) {
            for (const bullet of allBullets) {
                if (this.checkCollision(alien, bullet)) {
                    alien.destroy = true;
                }
            }
        }
    }
    getCanvas() {
        return this.canvas;
    }
    checkCollision(alien, bullet) {
        // Handle different shapes
        return this.circleRectCollision(alien.position, 10, bullet.position, 5, 10);
    }
    circleRectCollision(circlePos, circleRadius, rectPos, rectWidth, rectHeight) {
        const closestX = Math.max(rectPos.x, Math.min(circlePos.x, rectPos.x + rectWidth));
        const closestY = Math.max(rectPos.y, Math.min(circlePos.y, rectPos.y + rectHeight));
        const distanceX = circlePos.x - closestX;
        const distanceY = circlePos.y - closestY;
        const distanceSquared = (distanceX * distanceX) + (distanceY * distanceY);
        return distanceSquared < (circleRadius * circleRadius);
    }
    update() {
        this.collisions();
        for (const player of this.players) {
            player.update(this.ctx);
            for (const bullet of player.bullets) {
                bullet.update(this.ctx);
            }
            player.bullets = player.bullets.filter(bullet => !bullet.destroy);
        }
        this.aliens = this.aliens.filter(alien => !alien.destroy);
        for (const alien of this.aliens) {
            alien.update(this.ctx);
        }
    }
    draw() {
        // Clear canvas
        this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
        // Draw player
        for (const player of this.players) {
            player.draw(this.ctx);
            // Draw bullets
            for (const bullet of player.bullets) {
                bullet.draw(this.ctx);
            }
        }
        // Draw aliens
        for (const alien of this.aliens) {
            alien.draw(this.ctx);
        }
    }
}
exports.SpaceInvadersGame = SpaceInvadersGame;
