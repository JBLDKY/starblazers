import type p5 from 'p5';
/**
 * Manages the Frames Per Second (FPS) display and control for the game.
 */
export class FPSManager {
	private lastFrameTime: number = 0;
	private fps: number = 0;
	private frameCount: number = 0;
	private fpsDisplayTime: number = 0;
	private readonly fpsInterval: number = 1000 / 60;
	private readonly menuInputInterval: number = 100;
	private menuLastFrameTime: number = 0;

	/**
	 * Creates an FPSManager instance.
	 * @param {p} p - The canvas rendering context to draw the FPS display.
	 */
	constructor(private p: p5) {}

	/**
	 * Updates the frame count and calculates the FPS.
	 * @param {number} timestamp - The timestamp of the current frame.
	 */
	public update(timestamp: number): void {
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
	public shouldDraw(timestamp: number): boolean {
		const elapsed = timestamp - this.lastFrameTime;
		if (elapsed > this.fpsInterval) {
			this.lastFrameTime = timestamp - (elapsed % this.fpsInterval);
			return true;
		}
		return false;
	}

	public shouldProcessMenuInput(timestamp: number): boolean {
		const elapsed = timestamp - this.menuLastFrameTime;
		if (elapsed > this.menuInputInterval) {
			this.menuLastFrameTime = timestamp - (elapsed % this.fpsInterval);
			return true;
		}
		return false;
	}

	/**
	 * Draws the FPS value on the canvas.
	 * TODO: Fix
	 */
	public draw(): void {
		this.p.fill('white');
		// this.p.fillText(`FPS: ${this.fps}`, 10, 30);
	}
}
