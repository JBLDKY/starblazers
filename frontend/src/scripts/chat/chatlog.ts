import { ChatMessage } from "./chatmessage";
import { User } from "../user/user";

export class ChatLog {
	messages: ChatMessage[];

	constructor() {
		this.messages = [new ChatMessage(new User("jord"), "hello world")];
	}

	addMessage(message: ChatMessage): void {
		this.messages.push(message);
		// Update the UI if needed
	}

	// Render method to display messages, if you're managing UI rendering manually
	render(): void {
		// Logic to display chat messages
	}
}
