import type { BaseMenu } from '$lib/menu/base';
import { MainMenu } from '$lib/menu/main';
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
	Settings
}

export class MenuFactory {
	newMenu(p: p5, menuIndex: MenuIndex, inputHandler: InputHandler): BaseMenu {
		switch (menuIndex) {
			case MenuIndex.Main:
				return new MainMenu(p, inputHandler);
			case MenuIndex.Settings:
				return new SettingsMenu(p, inputHandler);
		}
	}
}
