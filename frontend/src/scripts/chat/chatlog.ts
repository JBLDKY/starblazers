import { ChatMessage } from "./chatmessage";
import { User } from "../user/user";

export class ChatLog {
	messages: ChatMessage[];

	constructor() {
		this.messages = [new ChatMessage(new User("jord"), "hello world")];
	}

	addMessage(message: ChatMessage): void {
		this.messages.push(message);
		this.render();
	}

	render(): void {
		const chatMessagesContainer = document.getElementById("chat-messages");
		if (chatMessagesContainer == null) {
			return;
		}

		const message = this.messages[this.messages.length - 1];

		const messageDiv = document.createElement("div");
		messageDiv.classList.add("chat-message");

		const messageContent = document.createTextNode(`${message.user.username}: ${message.text}`);

		messageDiv.appendChild(messageContent);
		chatMessagesContainer.appendChild(messageDiv);
		chatMessagesContainer.scrollTop = chatMessagesContainer.scrollHeight;
	}
}
