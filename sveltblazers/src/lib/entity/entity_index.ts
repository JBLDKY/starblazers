import { SomeoneElsesLobby } from '$lib/menu/SomeoneElsesLobby';
import type { BaseMenu } from '$lib/menu/base';
import { CurrentPlayerOwnLobbyMenu } from '$lib/menu/currentPlayerOwnLobby';
import { JoinLobbyMenu } from '$lib/menu/joinLobbyMenu';
import { MainMenu } from '$lib/menu/main';
import { MultiplayerMenu } from '$lib/menu/multiplayer';
import { SettingsMenu } from '$lib/menu/settings';
import { InputHandler } from '$lib/system/input_handler';
import type { WebSocketManager } from '$lib/websocketmanager';
import type p5 from 'p5';

export enum EntityIndex {
	Alien,
	slowStraightShootingAlien,
	Bullet,
	Player
}

export enum MenuKind {
	Undefined,
	Main,
	Settings,
	Multiplayer,
	CurrentPlayerOwnLobby,
	JoinLobby,
	SomeoneElsesLobby
}

export class MenuFactory {
	newMenu(
		p: p5,
		menuIndex: MenuKind,
		inputHandler: InputHandler,
		websocket: WebSocketManager,
		...args: string[]
	): BaseMenu {
		switch (menuIndex) {
			case MenuKind.Main:
				return new MainMenu(p, inputHandler);
			case MenuKind.Settings:
				return new SettingsMenu(p, inputHandler);
			case MenuKind.Multiplayer:
				return new MultiplayerMenu(p, inputHandler, websocket);
			case MenuKind.CurrentPlayerOwnLobby:
				return new CurrentPlayerOwnLobbyMenu(p, inputHandler, websocket);
			case MenuKind.JoinLobby:
				return new JoinLobbyMenu(p, inputHandler, websocket);
			case MenuKind.SomeoneElsesLobby:
				return new SomeoneElsesLobby(p, inputHandler, websocket, args[0][0]); // Provide the lobby name
		}
	}
}
