"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.Entity = void 0;
class Entity {
    constructor(position, speed) {
        this.position = position;
        this.speed = speed;
    }
    move(direction) {
        this.position.x += direction.x * this.speed;
    }
}
exports.Entity = Entity;
