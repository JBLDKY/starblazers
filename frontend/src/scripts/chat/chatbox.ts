import { ChatLog } from "./chatlog";
import { User } from "../user/user";
import { ChatInput } from "./chatinput";
import { ChatMessage } from "./chatmessage";

export class ChatBox {
	chatLog: ChatLog;
	chatInput: ChatInput;
	readonly user: User;

	constructor(user: User) {
		this.user = user;
		this.chatLog = new ChatLog();
		this.chatInput = new ChatInput(this.user, this.handleSendMessage.bind(this));
	}

	handleSendMessage(text: string): void {
		// Logic to send message to the server
		// For now, just add it to the chat history
		const userMessage = new ChatMessage(this.user, text); // Assume `currentUser` is the current user object
		this.chatLog.addMessage(userMessage);
		this.render();
		// websocket.send(userMessage)
	}

	render(): void {
		console.log(this.chatLog);
	}
}
