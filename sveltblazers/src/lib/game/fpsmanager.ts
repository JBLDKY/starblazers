/**
 * Manages the Frames Per Second (FPS) display and control for the game.
 */
export class FPSManager {
	private gameStartFrameTime: number = 0;
	private lastFrameTime: number = 0;
	private frameCount: number = 0;
	private fpsDisplayTime: number = 0;
	private readonly fpsInterval: number = 1000 / 60;
	private readonly menuInputInterval: number = 100;
	private menuLastFrameTime: number = 0;

	/**
	 * Creates an FPSManager instance.
	 */
	constructor() {}

	/**
	 * Updates the frame count and calculates the FPS.
	 * @param {number} timestamp - The timestamp of the current frame.
	 */
	public update(timestamp: number): void {
		if (timestamp - this.fpsDisplayTime > 1000) {
			this.frameCount = 0;
			this.fpsDisplayTime = timestamp;
		}
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
	 * Determines whether menu input should be processed based on the time elapsed since the last input processing.
	 * This method helps in throttling the menu input to avoid excessively frequent processing.
	 *
	 * @param {number} timestamp - The current time in milliseconds, typically received from a high-resolution timer.
	 * @returns {boolean} True if the input should be processed, false otherwise.
	 */
	public shouldProcessMenuInput(timestamp: number): boolean {
		const elapsed = timestamp - this.menuLastFrameTime; // Calculate the time elapsed since the last processed frame
		if (elapsed > this.menuInputInterval) {
			// Check if the elapsed time exceeds the set interval for menu input
			// Adjust the last frame time, compensating for any extra time beyond the interval
			this.menuLastFrameTime = timestamp - (elapsed % this.menuInputInterval);
			return true; // Return true to indicate that input should be processed
		}
		return false; // Return false if the interval has not yet been exceeded
	}

	public getFrameCount(): number {
		return this.frameCount;
	}

	public getInGameTime(): number {
		return this.lastFrameTime - this.gameStartFrameTime;
	}
}
