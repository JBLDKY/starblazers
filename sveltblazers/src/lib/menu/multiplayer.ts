import type p5 from 'p5';
import { BaseMenu } from './base';
import { Navigator } from './navigator';

import {
	MENU_STARTING_Y_COORDINATE,
	HEADER_SIZE,
	ITEM_SIZE,
	PIXELS_BELOW_MAIN_MENU,
	PIXELS_BETWEEN_ITEMS,
	MULTIPLAYER_MENU,
	MULTIPLAYER_MENU_ITEM_TEXTS
} from './menuConstants';
import type { InputHandler } from '$lib/system/input_handler';
import type { WebSocketManager } from '$lib/websocketmanager';

/**
 * Represents a Multiplayer menu derived from the BaseMenu. This class manages the creating & joining of lobbies.
 */
export class MultiplayerMenu extends BaseMenu {
	private currentY: number;

	/**
	 * Constructs a multiplayer menu with given p5 instance.
	 * @param {p5} p - The p5 instance used for drawing the menu.
	 */
	constructor(p: p5, inputHandler: InputHandler, websocket: WebSocketManager) {
		super(p, inputHandler);
		this.p = p;
		this.p.fill('deeppink');
		this.currentY = MENU_STARTING_Y_COORDINATE;
		this.websocket = websocket;

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
				.setLabel(MULTIPLAYER_MENU)
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

		MULTIPLAYER_MENU_ITEM_TEXTS.forEach((itemText) => {
			this.items.push(this.builder.setLabel(itemText).setAbsoluteY(this.currentY).build());
			this.currentY += PIXELS_BETWEEN_ITEMS;
		});
	}

	private createLobby(): void {
		if (this.websocket === null || this.websocket === undefined) {
			console.error('Could not create lobby');
			return;
		}

		this.websocket.sendMessage(
			JSON.stringify({
				type: 'CreateLobby',
				lobby_name: this.playerInfo.uuid + "'s lobby",
				player_id: this.playerInfo.uuid
			})
		);
	}

	loop = (_: number): void => {
		const result = this.handleInput(this.inputHandler.getCachedKeyPresses());
		if (result == 'Create lobby') {
			this.createLobby();
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
