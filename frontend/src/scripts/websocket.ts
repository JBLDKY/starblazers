// type MessageListener = (this: WebSocket, ev: MessageEvent<any>) => any;
//
// class WebSocket {
// 	private socket: WebSocket | null = null;
// 	private readonly uri: string;
// 	private messageListeners: MessageListener[] = [];
// 	constructor(uri: string) {
// 		this.uri = uri;
// 	}
//
// 	public connect() {
// 		this.socket = new WebSocket(this.uri);
//
// 		this.socket.onopen = (event) => {
// 			console.log("WebSocket connection established", event);
// 			// Handle WebSocket upgrade and any initialization post-connection
// 		};
//
// 		this.socket.onmessage = (event) => {
// 			console.log("Message from server ", event.data);
// 			this.messageListeners.forEach((listener) => listener.call(this.socket as WebSocket, event));
// 		};
//
// 		this.socket.onclose = (event) => {
// 			console.log("WebSocket connection closed", event);
// 			// Handle cleanup if necessary
// 		};
//
// 		this.socket.onerror = (event) => {
// 			console.error("WebSocket error observed:", event);
// 			// Handle errors appropriately
// 		};
// 	}
//
// 	public disconnect() {
// 		if (this.socket) {
// 			this.socket.close();
// 		}
// 	}
//
// 	public sendMessage(message: string) {
// 		if (this.socket && this.socket.readyState === WebSocket.OPEN) {
// 			this.socket.send(message);
// 		} else {
// 			console.error("WebSocket is not connected.");
// 		}
// 	}
//
// 	public addMessageListener(listener: MessageListener) {
// 		this.messageListeners.push(listener);
// 	}
//
// 	public removeMessageListener(listener: MessageListener) {
// 		this.messageListeners = this.messageListeners.filter((l) => l !== listener);
// 	}
// }
//
// export default WebSocketService;
