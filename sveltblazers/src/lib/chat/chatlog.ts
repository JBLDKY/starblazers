import { ChatMessage } from './chatmessage';
import { chatLogStore } from '../../store/chat';

export class ChatLog {
	messages: ChatMessage[];

	constructor() {
		this.messages = [];
	}

	addMessage(message: ChatMessage): void {
		// Store message objects in our class
		this.messages.push(message);

		// Format our each ChatMessage object as a string
		const messages = this.messages.map((msg) => `${msg.user.username}: ${msg.text}`);

		// Store the strings in the svelte store
		chatLogStore.set(messages);
	}
}
