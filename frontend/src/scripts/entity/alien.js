"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Alien = void 0;
const base_1 = require("./base");
class Alien extends base_1.Entity {
    constructor(position, speed) {
        super(position, speed);
        this.cycle = 0;
        this.moveDown = false;
        this.xVelocity = 30;
        this.destroy = false;
    }
    update(ctx) {
        if (this.moveDown) {
            this.position.y += 30; // Move down
            this.xVelocity *= -1; // turn around (horizontally)
            this.moveDown = false;
        }
        else {
            this.position.x += this.speed * Number(this.xVelocity);
        }
        // Check if at the edge of the canvas and need to move down 
        if (this.position.x <= 0 && this.xVelocity < 0 || this.position.x >= ctx.canvas.width && this.xVelocity > 0) {
            this.moveDown = true;
        }
    }
    draw(ctx) {
        ctx.beginPath();
        ctx.arc(this.position.x, this.position.y, 10, 0, 2 * Math.PI);
        ctx.fillStyle = "green";
        ctx.fill();
    }
    ;
}
exports.Alien = Alien;
