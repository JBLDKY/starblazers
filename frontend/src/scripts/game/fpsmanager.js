"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.FPSManager = void 0;
/**
 * Manages the Frames Per Second (FPS) display and control for the game.
 */
class FPSManager {
    /**
     * Creates an FPSManager instance.
     * @param {CanvasRenderingContext2D} ctx - The canvas rendering context to draw the FPS display.
     */
    constructor(ctx) {
        this.ctx = ctx;
        this.lastFrameTime = 0;
        this.fps = 0;
        this.frameCount = 0;
        this.fpsDisplayTime = 0;
        this.fpsInterval = 1000 / 60;
    }
    /**
     * Updates the frame count and calculates the FPS.
     * @param {number} timestamp - The timestamp of the current frame.
     */
    update(timestamp) {
        if (timestamp - this.fpsDisplayTime > 1000) {
            this.fps = this.frameCount;
            this.frameCount = 0;
            this.fpsDisplayTime = timestamp;
        }
        // TODO: What happens when values like these exceed their capacity in javascript?
        // TODO: Can values like these be set to wrap?
        this.frameCount++;
    }
    /**
     * Determines if a new frame should be drawn based on the FPS interval.
     * @param {number} timestamp - The timestamp of the current frame.
     * @returns {boolean} True if a new frame should be drawn, false otherwise.
     */
    shouldDraw(timestamp) {
        const elapsed = timestamp - this.lastFrameTime;
        if (elapsed > this.fpsInterval) {
            this.lastFrameTime = timestamp - (elapsed % this.fpsInterval);
            return true;
        }
        return false;
    }
    /**
     * Draws the FPS value on the canvas.
     */
    draw() {
        this.ctx.fillStyle = "white";
        this.ctx.font = "20px Arial";
        this.ctx.fillText(`FPS: ${this.fps}`, 10, 30);
    }
}
exports.FPSManager = FPSManager;
