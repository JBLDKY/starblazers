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
	JOIN_LOBBY_MENU,
	JOIN_LOBBY_MENU_ITEM_TEXTS
} from './menuConstants';
import type { InputHandler } from '$lib/system/input_handler';
import { get } from 'svelte/store';
import { jwtStore } from '../../store/auth';
import { GAME_LOBBIES_URL } from '../../constants';

/**
 * Represents a Multiplayer menu derived from the BaseMenu. This class manages the creating & joining of lobbies.
 */
export class JoinLobbyMenu extends BaseMenu {
	private builder: MenuItemBuilder;
	private currentY: number;
	private lastUpdate = 0;

	/**
	 * Constructs a multiplayer menu with given p5 instance.
	 * @param {p5} p - The p5 instance used for drawing the menu.
	 */
	constructor(p: p5, inputHandler: InputHandler) {
		super(p, inputHandler);
		this.p = p;
		this.p.fill('deeppink');
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
				.setLabel(JOIN_LOBBY_MENU)
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

		JOIN_LOBBY_MENU_ITEM_TEXTS.forEach((itemText) => {
			this.items.push(this.builder.setLabel(itemText).setAbsoluteY(this.currentY).build());
			this.currentY += PIXELS_BETWEEN_ITEMS;
		});
	}

	async updateLobbies() {
		try {
			const lobbies = await this.getLobbies();
			console.log('Lobbies:', lobbies);
			// Update your game state with the fetched lobbies
		} catch (error) {
			console.error('Error updating lobbies:', error);
			// Handle the error appropriately in your game
		}
	}

	async getLobbies(): Promise<string[]> {
		const jwt = get(jwtStore);

		try {
			const response = await fetch(GAME_LOBBIES_URL, {
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

			const lobbies = await response.json();
			return lobbies;
		} catch (error) {
			console.error('Fetch error:', error);
			throw error; // Re-throw the error to be caught by the caller
		}
	}

	loop = (timestamp: number): void => {
		const result = this.handleInput(this.inputHandler.getCachedKeyPresses());

		if (timestamp - this.lastUpdate > 3000) {
			// Chech for new lobbies every 3 seconds
			this.updateLobbies();
			this.lastUpdate = timestamp;
		}

		if (result != '' && result != undefined) {
			this.inputHandler.handleMenuResult(result);
		}

		this.p.clear();
		this.display();
		this.p.rect(0, 30, 30, this.p.height);
		this.p.rect(this.p.width - 30, 30, 30, this.p.height);
	};
}
