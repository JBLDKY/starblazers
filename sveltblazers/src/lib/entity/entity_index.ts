import { SomeoneElsesLobby } from '$lib/menu/SomeoneElsesLobby';
import type { BaseMenu } from '$lib/menu/base';
import { CurrentPlayerOwnLobbyMenu } from '$lib/menu/currentPlayerOwnLobby';
import { JoinLobbyMenu } from '$lib/menu/joinLobbyMenu';
import { MainMenu } from '$lib/menu/main';
import { MultiplayerMenu } from '$lib/menu/multiplayer';
import { SettingsMenu } from '$lib/menu/settings';
import { InputHandler } from '$lib/system/input_handler';
import type p5 from 'p5';

export enum EntityIndex {
	Alien,
	slowStraightShootingAlien,
	Bullet,
	Player
}

export enum MenuIndex {
	Main,
	Settings,
	Multiplayer,
	CurrentPlayerOwnLobby,
	JoinLobby,
	SomeoneElsesLobby
}

export class MenuFactory {
	newMenu(p: p5, menuIndex: MenuIndex, inputHandler: InputHandler, ...args: string[]): BaseMenu {
		switch (menuIndex) {
			case MenuIndex.Main:
				return new MainMenu(p, inputHandler);
			case MenuIndex.Settings:
				return new SettingsMenu(p, inputHandler);
			case MenuIndex.Multiplayer:
				return new MultiplayerMenu(p, inputHandler);
			case MenuIndex.CurrentPlayerOwnLobby:
				return new CurrentPlayerOwnLobbyMenu(p, inputHandler, ...args);
			case MenuIndex.JoinLobby:
				return new JoinLobbyMenu(p, inputHandler);
			case MenuIndex.SomeoneElsesLobby:
				return new SomeoneElsesLobby(p, inputHandler, ...args);
		}
	}
}
