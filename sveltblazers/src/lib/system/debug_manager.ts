export class DebugManager {
	static debugMode: boolean = false;

	static toggleDebugMode() {
		DebugManager.debugMode = !DebugManager.debugMode;
		console.log(`Debug mode is now ${DebugManager.debugMode ? 'ON' : 'OFF'}`);
	}
}

export default DebugManager;
