"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Player = void 0;
const base_1 = require("./base");
const bullet_1 = require("./bullet");
class Player extends base_1.Entity {
    constructor(position, speed) {
        super(position, speed);
        this.keyPresses = {};
        this.handleKeyDown = (event) => {
            this.keyPresses[event.key] = true;
            console.log(event);
        };
        this.handleKeyUp = (event) => {
            this.keyPresses[event.key] = false;
        };
        this.bullets = [];
        this.fireRate = 5;
        this.cycles = 0;
        document.addEventListener('keydown', this.handleKeyDown);
        document.addEventListener('keyup', this.handleKeyUp);
    }
    ;
    update(ctx) {
        if (this.keyPresses['w']) {
            this.position.y -= this.speed;
        }
        if (this.keyPresses['s']) {
            this.position.y += this.speed;
        }
        if (this.keyPresses['a']) {
            this.position.x -= this.speed;
        }
        if (this.keyPresses['d']) {
            this.position.x += this.speed;
        }
        if (this.bullets.length < 100 && this.cycles % this.fireRate == 0) {
            this.fire();
        }
        this.position.x = Math.max(0, Math.min(this.position.x, ctx.canvas.width));
        this.position.y = Math.max(0, Math.min(this.position.y, ctx.canvas.height));
        this.cycles += 1;
    }
    ;
    draw(ctx) {
        ctx.beginPath();
        ctx.moveTo(this.position.x, this.position.y);
        ctx.lineTo(this.position.x - 10, this.position.y + 20);
        ctx.lineTo(this.position.x + 10, this.position.y + 20);
        ctx.closePath();
        ctx.fillStyle = "pink";
        ctx.fill();
    }
    ;
    fire() {
        let x = this.position.x;
        let y = this.position.y;
        let position = { x, y };
        this.bullets.push(new bullet_1.Bullet(position, 10));
    }
    ;
    destroy() {
        document.removeEventListener('keydown', this.handleKeyDown);
        document.removeEventListener('keyup', this.handleKeyUp);
    }
}
exports.Player = Player;
;
