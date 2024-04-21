export class User {
	uuid: string;
	username: string;

	constructor(username: string) {
		this.uuid = this.generateUUID();
		this.username = username;
	}

	private generateUUID(): string {
		// Implementation to generate a unique identifier
		return 'uuid-12345'; // Placeholder, use a proper UUID generation method
	}
}
