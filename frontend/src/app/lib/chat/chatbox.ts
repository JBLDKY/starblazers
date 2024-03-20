import { ChatLog } from "./chatlog";
import { User } from "../user/user";
import { ChatInput } from "./chatinput";
import { ChatMessage } from "./chatmessage";
import { WebSocketManager } from "../websocketmanager";

export class ChatBox {
	chatLog: ChatLog;
	chatInput: ChatInput;
	websocket: WebSocketManager;
	readonly user: User;

	constructor(user: User, websocket: WebSocketManager) {
		this.user = user;
		this.chatLog = new ChatLog();
		this.chatInput = new ChatInput(this.user, this.handleSendMessage.bind(this));
		this.websocket = websocket;
	}

	handleSendMessage(text: string): void {
		// Logic to send message to the server
		// For now, just add it to the chat history
		const userMessage = new ChatMessage(this.user, text);
		this.chatLog.addMessage(userMessage);

		this.websocket.sendMessage(text);
	}

	getChatInputElement(): HTMLInputElement | null {
		return this.chatInput.element();
	}

	getChatLineElement(): HTMLElement | null {
		return document.getElementById("chat-line");
	}

	receiveMessage() {
		const websocketMessages = this.websocket.getMessages();

		if (this.chatLog.messages.length != websocketMessages.length) {
			const message = websocketMessages[websocketMessages.length - 1];
			if (message == undefined) {
				return;
			}

			if (this.user.username == "" && message.includes("YOUR ID IS:")) {
				this.user.username = message.split(": ")[1];
				this.websocket.messages.pop();
				return;
			}

			const split = message.split(":");
			const u = split[0];
			const t = split[1];

			const user = new User(u);
			const msg = new ChatMessage(user, t);
			this.chatLog.addMessage(msg);
		}
		// if (this.chatLog.texts.length != websocketMessages.length) {
		// 	this.chatLog.texts.push(websocketMessages[websocketMessages.length - 1]);
		// 	console.log("new message!", websocketMessages[websocketMessages.length - 1]);
		// }
	}

	sendMessage() {
		const chatInput = this.getChatInputElement();
		if (chatInput == null || chatInput.value.trim() == "") {
			return;
		}

		// Actual sending happens here
		this.chatInput.handleInput(chatInput.value);

		// Set the message empty string and unfocus the input field
		chatInput.value = "";
		chatInput.blur();

		const chatLine = this.getChatLineElement();
		if (chatLine != null) {
			chatLine.style.display = "none";
		}

		const game = document.getElementById("game-canvas");
		if (game != null) {
			game.focus();
		}
	}

	/*
	 * Start typing a new message.
	 *
	 * Also enables some styling on the chatbox, the cute chatline
	 */
	startMessage() {
		const chatInput = this.getChatInputElement();
		if (chatInput != null) {
			// This allows the user to start typing
			chatInput.focus();
		}

		const chatLine = this.getChatLineElement();
		if (chatLine != null) {
			chatLine.style.display = "block";
		}
	}

	/*
	 * Cancels the message the user was typing and returns to the game.
	 *
	 * Message will not be sent and the chat input will be cleared.
	 */
	cancelMessage() {
		const chatInput = this.getChatInputElement();
		if (chatInput != null && chatInput == document.activeElement) {
			chatInput.value = ""; // clear the message the user was typing
			chatInput.blur(); // unfocus
		}

		const chatLine = this.getChatLineElement();
		if (chatLine != null) {
			chatLine.style.display = "none"; // Hide our cute chatline
		}

		const game = document.getElementById("game-canvas");
		if (game != null) {
			game.focus(); // go back to the game
		}
	}
}
