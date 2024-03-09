"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Bullet = void 0;
const base_1 = require("./base");
const color_1 = require("../assets/color");
class Bullet extends base_1.Entity {
    constructor(position, speed) {
        super(position, speed);
        this.destroy = false;
        this.yVelocity = 1;
    }
    draw(ctx) {
        ctx.fillStyle = color_1.Colors.EFFECT;
        ctx.fillRect(this.position.x, this.position.y, 5, 10);
    }
    update() {
        this.position.y -= 1 * this.speed * this.yVelocity;
        if (this.position.y < 0) {
            this.destroy = true;
        }
    }
}
exports.Bullet = Bullet;
