"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Bullet = void 0;
const base_1 = require("./base");
class Bullet extends base_1.Entity {
    constructor(position, speed) {
        super(position, speed);
        this.destroy = false;
        this.yVelocity = 1;
    }
    draw(ctx) {
        ctx.fillStyle = "red";
        ctx.fillRect(this.position.x, this.position.y, 5, 10);
    }
    ;
    update(ctx) {
        this.position.y -= 1 * this.speed * this.yVelocity;
        if (this.position.y < 0) {
            this.destroy = true;
        }
    }
}
exports.Bullet = Bullet;
