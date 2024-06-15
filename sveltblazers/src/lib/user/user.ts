export class User {
	uuid: string;
	username: string;

	constructor(username: string, player_id: string) {
		this.uuid = player_id;
		this.username = username;
	}
}
