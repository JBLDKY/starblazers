import { ChatMessage } from './chatmessage';

export class ChatLog {
	messages: ChatMessage[];
	texts: string[];

	constructor() {
		this.messages = [];
		this.texts = [];
	}

	addMessage(message: ChatMessage): void {
		this.messages.push(message);
		this.render();
	}

	render(): void {
		const chatMessagesContainer = document.getElementById('chat-messages');
		if (chatMessagesContainer == null) {
			return;
		}

		const message = this.messages[this.messages.length - 1];

		const messageDiv = document.createElement('div');
		messageDiv.classList.add('chat-message');

		const messageContent = document.createTextNode(`${message.user.username}: ${message.text}`);

		messageDiv.appendChild(messageContent);
		chatMessagesContainer.appendChild(messageDiv);
		chatMessagesContainer.scrollTop = chatMessagesContainer.scrollHeight;
	}
}
