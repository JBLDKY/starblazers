import type p5 from 'p5';
import { BaseMenu } from './base';
import { MenuItemBuilder } from './menuitem/menu_item_builder';
import { Navigator } from './navigator';

import {
	MENU_STARTING_Y_COORDINATE,
	MAIN_MENU,
	HEADER_SIZE,
	ITEM_SIZE,
	PIXELS_BELOW_MAIN_MENU,
	PIXELS_BETWEEN_ITEMS,
	MAIN_MENU_ITEM_TEXTS
} from './menuConstants';
import type { InputHandler } from '$lib/system/input_handler';

/**
 * Represents a main menu derived from the BaseMenu. This class manages the
 * layout and interaction of a settings menu, including creating headers and items.
 */
export class MainMenu extends BaseMenu {
	private builder: MenuItemBuilder;
	private currentY: number;

	/**
	 * Constructs a settings menu with given p5 instance.
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
	 * Creates the header for the settings menu, setting the title and its initial position.
	 */
	private createHeader(): void {
		this.items.push(
			this.builder
				.setLabel(MAIN_MENU)
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

		MAIN_MENU_ITEM_TEXTS.forEach((itemText) => {
			this.items.push(this.builder.setLabel(itemText).setAbsoluteY(this.currentY).build());
			this.currentY += PIXELS_BETWEEN_ITEMS;
		});
	}

	/**
	 * Handles user input to navigate through menu items or to execute actions based on the current selection.
	 * @param {Object} cachedKeyPresses - A key-value map where each key is a button press and its value is a boolean indicating if it was pressed.
	 * @returns {string} - The label of the selected menu item, or an empty string if no actionable input was detected.
	 */
	handleInput(cachedKeyPresses: { [key: string]: boolean }): string {
		if (cachedKeyPresses['w'] || cachedKeyPresses['k']) {
			cachedKeyPresses['w'] = false;
			cachedKeyPresses['k'] = false;
			this.prevItem();
		}

		if (cachedKeyPresses['s'] || cachedKeyPresses['j']) {
			cachedKeyPresses['s'] = false;
			cachedKeyPresses['j'] = false;
			this.nextItem();
		}

		if (cachedKeyPresses['Enter']) {
			cachedKeyPresses['Enter'] = false;

			const selected = this.navigator.currentItem();
			if (selected != null) {
				return selected.getLabel();
			}

			console.error(
				'selected an empty menu item at index: ',
				this.index,
				' available range: ',
				0,
				' ',
				this.items.length - 1
			);
		}

		return '';
	}

	loop = (): void => {
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
