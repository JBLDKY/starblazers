import p5 from 'p5-svelte';
/**
 * Manages the Frames Per Second (FPS) display and control for the game.
 */
export class FPSManager {
	private lastFrameTime: number = 0;
	private fps: number = 0;
	private frameCount: number = 0;
	private fpsDisplayTime: number = 0;
	private readonly fpsInterval: number = 1000 / 60;

	/**
	 * Creates an FPSManager instance.
	 * @param {p5} p5 - The canvas rendering context to draw the FPS display.
	 */
	constructor(private p5: p5) {}

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

	/**
	 * Draws the FPS value on the canvas.
	 */
	public draw(): void {
		this.p5.fillStyle = 'white';
		this.p5.font = '20px Arial';
		// this.p5.fillText(`FPS: ${this.fps}`, 10, 30);
	}
}
