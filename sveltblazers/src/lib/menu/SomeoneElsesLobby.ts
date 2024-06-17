import type p5 from 'p5';
import { BaseMenu } from './base';
import { Navigator } from './navigator';
import { playerInfoStore } from '../../store/auth';

import {
	MENU_STARTING_Y_COORDINATE,
	HEADER_SIZE,
	ITEM_SIZE,
	PIXELS_BELOW_MAIN_MENU,
	PIXELS_BETWEEN_ITEMS,
	SOMEONE_ELSES_LOBBY_MENU,
	SOMEONE_ELSES_LOBBY_MENU_ITEM_TEXTS
} from './menuConstants';
import type { InputHandler } from '$lib/system/input_handler';
import { get } from 'svelte/store';
import { jwtStore } from '../../store/auth';
import { get_players_in_lobby_url } from '../../constants';
import type { WebSocketManager } from '$lib/websocketmanager';
import type { PublicPlayerData } from '../../routes/helpers';

/**
 * Represents a Multiplayer menu derived from the BaseMenu. This class manages the creating & joining of lobbies.
 */
export class SomeoneElsesLobby extends BaseMenu {
	private currentY: number;
	private lastUpdate = 0;
	private players: string[] = [];
	private playerInfo: PublicPlayerData;
	private lobbyName: string;

	/**
	 * Constructs a multiplayer menu with given p5 instance.
	 * @param {p5} p - The p5 instance used for drawing the menu.
	 */
	constructor(p: p5, inputHandler: InputHandler, websocket: WebSocketManager, lobbyName: string) {
		super(p, inputHandler);
		this.p = p;
		this.p.fill('deeppink');
		this.currentY = MENU_STARTING_Y_COORDINATE;
		this.lobbyName = lobbyName;

		this.createHeader();
		this.createItems();

		this.playerInfo = get(playerInfoStore);

		this.websocket = websocket;
		this.navigator = new Navigator(this.p);
		this.navigator.moveTo(this.items[this.index]);
	}

	/**
	 * Creates the header for the multiplayer menu, setting the title and its initial position.
	 */
	private createHeader(): void {
		this.items.push(
			this.builder
				.setLabel(SOMEONE_ELSES_LOBBY_MENU)
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

		SOMEONE_ELSES_LOBBY_MENU_ITEM_TEXTS.forEach((itemText) => {
			this.items.push(this.builder.setLabel(itemText).setAbsoluteY(this.currentY).build());
			this.currentY += PIXELS_BETWEEN_ITEMS;
		});
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
			const lobby_name_without_suffix = this.lobbyName.replace("'s lobby", '');
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

	private recreate(): void {
		this.items = [];
		this.currentY = MENU_STARTING_Y_COORDINATE;
		this.createHeader();
		this.createItems();
		this.players.forEach((player) => {
			if (this.playerInfo.uuid == player) {
				player = player + ' (you)';
			}
			this.addItem(player);
		});
	}

	onExit(): void {
		this.inputHandler.handleMenuResult('LeaveLobby');
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
}
