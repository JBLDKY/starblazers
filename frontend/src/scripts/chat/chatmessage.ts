import { User } from "../user/user";

export class ChatMessage {
	user: User;
	text: string;

	constructor(user: User, text: string) {
		this.user = user;
		this.text = text;
	}
}
