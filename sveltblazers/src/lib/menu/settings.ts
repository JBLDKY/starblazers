import type p5 from 'p5';
import { BaseMenu } from './base';

import {
	MENU_STARTING_Y_COORDINATE,
	SETTINGS_MENU,
	HEADER_SIZE,
	ITEM_SIZE,
	PIXELS_BELOW_MAIN_MENU,
	PIXELS_BETWEEN_ITEMS,
	OPTION_MENU_ITEM_TEXTS
} from './menuConstants';
import type { InputHandler } from '$lib/system/input_handler';
import { MenuKind } from '$lib/entity/entity_index';

/**
 * Represents a settings menu derived from the BaseMenu. This class manages the
 * layout and interaction of a settings menu, including creating headers and items.
 */
export class SettingsMenu extends BaseMenu {
	private currentY: number;

	public kind: MenuKind = MenuKind.Settings;

	/**
	 * Constructs a settings menu with given p5 instance.
	 * @param {p5} p - The p5 instance used for drawing the menu.
	 */
	constructor(p: p5, inputHandler: InputHandler) {
		super(p, inputHandler);
		this.p = p;
		this.p.fill('deeppink');
		this.currentY = MENU_STARTING_Y_COORDINATE;

		this.createHeader();
		this.createItems();
	}

	/**
	 * Creates the header for the settings menu, setting the title and its initial position.
	 */
	private createHeader(): void {
		this.items.push(
			this.builder
				.setLabel(SETTINGS_MENU)
				.setTextSize(HEADER_SIZE)
				.setRelativeX(0.5)
				.setAbsoluteY(this.currentY)
				.build()
		);

		this.currentY += PIXELS_BELOW_MAIN_MENU;
	}

	/**
	 * Dynamically creates menu items based on constants defined for the settings menu.
	 */
	private createItems(): void {
		this.builder.setTextSize(ITEM_SIZE);

		OPTION_MENU_ITEM_TEXTS.forEach((itemText) => {
			this.items.push(this.builder.setLabel(itemText).setAbsoluteY(this.currentY).build());
			this.currentY += PIXELS_BETWEEN_ITEMS;
		});
	}

	loop = (_: number): void => {
		const result = this.handleInput(this.inputHandler.getCachedKeyPresses());

		if (result != '' && result != undefined) {
			this.inputHandler.handleMenuResult(result);
		}

		this.p.clear();
		this.display();
		this.p.rect(0, 30, 30, this.p.height);
		this.p.rect(this.p.width - 30, 30, 30, this.p.height);
	};
}
