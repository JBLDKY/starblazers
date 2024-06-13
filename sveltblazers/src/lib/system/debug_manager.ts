export class DebugManager {
	static debugMode: boolean = false;

	static toggleDebugMode() {
		DebugManager.debugMode = !DebugManager.debugMode;
	}
}

export default DebugManager;
