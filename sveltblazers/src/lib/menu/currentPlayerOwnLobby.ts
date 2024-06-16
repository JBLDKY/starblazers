import type p5 from 'p5';
import { BaseMenu } from './base';
import { MenuItemBuilder } from './menuitem/menu_item_builder';
import { Navigator } from './navigator';

import {
	MENU_STARTING_Y_COORDINATE,
	HEADER_SIZE,
	ITEM_SIZE,
	PIXELS_BELOW_MAIN_MENU,
	PIXELS_BETWEEN_ITEMS,
	CURRENT_PLAYER_OWN_LOBBY_MENU,
	CURRENT_PLAYER_OWN_LOBBY_MENU_ITEM_TEXTS
} from './menuConstants';
import type { InputHandler } from '$lib/system/input_handler';
import { get } from 'svelte/store';
import { jwtStore } from '../../store/auth';
import { get_players_in_lobby_url } from '../../constants';

/**
 * Represents a Multiplayer menu derived from the BaseMenu. This class manages the creating & joining of lobbies.
 */
export class CurrentPlayerOwnLobbyMenu extends BaseMenu {
	private builder: MenuItemBuilder;
	private currentY: number;
	private lobbyName: string;
	private players: string[] = [];
	private lastUpdate: number = 0;

	/**
	 * Constructs a multiplayer menu with given p5 instance.
	 * @param {p5} p - The p5 instance used for drawing the menu.
	 */
	constructor(p: p5, inputHandler: InputHandler, lobbyName: string[]) {
		super(p, inputHandler);
		this.p = p;
		this.p.fill('deeppink');
		this.lobbyName = lobbyName[0];
		this.currentY = MENU_STARTING_Y_COORDINATE;

		this.builder = new MenuItemBuilder(this.p);

		this.createHeader();
		this.createItems();

		this.navigator = new Navigator(this.p);
		this.navigator.moveTo(this.items[this.index]);
	}

	/**
	 * Creates the header for the multiplayer menu, setting the title and its initial position.
	 */
	private createHeader(): void {
		this.items.push(
			this.builder
				.setLabel(CURRENT_PLAYER_OWN_LOBBY_MENU)
				.setTextSize(HEADER_SIZE)
				.setRelativeX(0.5)
				.setAbsoluteY(this.currentY)
				.build()
		);

		this.currentY += PIXELS_BELOW_MAIN_MENU;
	}

	/**
	 * Dynamically creates menu items based on constants defined for the multiplayer menu.
	 */
	private createItems(): void {
		this.builder.setTextSize(ITEM_SIZE);

		CURRENT_PLAYER_OWN_LOBBY_MENU_ITEM_TEXTS.forEach((itemText) => {
			this.items.push(this.builder.setLabel(itemText).setAbsoluteY(this.currentY).build());
			this.currentY += PIXELS_BETWEEN_ITEMS;
		});
	}

	async updatePlayersInLobby() {
		try {
			const players = await this.getPlayersInLobby();
			this.players = players;
		} catch (error) {
			console.error('Error updating lobbies:', error);
			return [];
		}
	}

	async getPlayersInLobby(): Promise<string[]> {
		const jwt = get(jwtStore);

		try {
			const lobby_name_without_suffix = this.lobbyName.split("'")[0];
			const url = get_players_in_lobby_url(lobby_name_without_suffix);
			const response = await fetch(url, {
				method: 'GET',
				mode: 'cors', // no-cors, *cors, same-origin
				headers: {
					Authorization: `Bearer ${jwt}`
				}
			});

			if (!response.ok) {
				switch (response.status) {
					case 401:
						console.warn('Token has been tampered with or has expired');
						throw new Error('Unauthorized');
					default:
						console.error('Server unavailable');
						throw new Error('Server unavailable');
				}
			}

			const players = await response.json();
			return players;
		} catch (error) {
			console.error('Fetch error:', error);
			throw error;
		}
	}

	private addItem(item: string): void {
		this.items.push(
			this.builder
				.setLabel(item)
				.setTextSize(ITEM_SIZE)
				.setRelativeX(0.5)
				.setAbsoluteY(this.currentY)
				.build()
		);
		this.currentY += PIXELS_BETWEEN_ITEMS;
	}

	private recreate(): void {
		this.items = [];
		this.currentY = MENU_STARTING_Y_COORDINATE;
		this.createHeader();
		this.createItems();
		this.players.forEach((player) => {
			if (this.lobbyName.includes(player)) {
				player = player + ' (you)';
			}
			this.addItem(player);
		});
	}

	loop = (timestamp: number): void => {
		const result = this.handleInput(this.inputHandler.getCachedKeyPresses());

		if (timestamp - this.lastUpdate > 3000) {
			// Check for new lobbies every 3 seconds
			this.updatePlayersInLobby();
			this.lastUpdate = timestamp;
		}

		if (result != '' && result != undefined) {
			this.inputHandler.handleMenuResult(result);
		}

		this.recreate();

		this.p.clear();
		this.display();
		this.p.rect(0, 30, 30, this.p.height);
		this.p.rect(this.p.width - 30, 30, 30, this.p.height);
	};

	onExit = (): void => {
		this.inputHandler.handleMenuResult('LeaveOwnLobby');
	};
}
