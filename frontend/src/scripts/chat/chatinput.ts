import { User } from "../user/user";

export class ChatInput {
	user: User;
	onSendMessage: (text: string) => void;

	constructor(user: User, onSendMessage: (text: string) => void) {
		this.user = user;
		this.onSendMessage = onSendMessage;
	}

	// Method to capture input and call onSendMessage
	// Bind this to an input field in your UI
	handleInput(text: string): void {
		if (text.trim() !== "") {
			this.onSendMessage(text);
		}
	}

	element(): HTMLInputElement | null {
		return document.getElementById("chatInput") as HTMLInputElement;
	}
}
