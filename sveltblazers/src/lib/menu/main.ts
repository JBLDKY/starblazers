import type p5 from 'p5';
import { BaseMenu } from './base';
import { MenuItemBuilder } from './menuitem/menu_item_builder';
import { Navigator } from './navigator';
import type { MenuItem } from './menuitem/menu_item';

import {
	MENU_STARTING_Y_COORDINATE,
	MAIN_MENU,
	HEADER_SIZE,
	ITEM_SIZE,
	PIXELS_BELOW_MAIN_MENU,
	PIXELS_BETWEEN_ITEMS,
	MAIN_MENU_ITEM_TEXTS
} from './menuConstants';

export class MainMenu extends BaseMenu {
	private items: MenuItem[] = [];
	private builder: MenuItemBuilder;
	private currentY: number;
	private navigator: Navigator;
	private index: number = 1;

	constructor(p: p5) {
		super(p);
		this.p = p;
		this.p.fill('deeppink');
		this.currentY = MENU_STARTING_Y_COORDINATE;

		this.builder = new MenuItemBuilder(this.p);

		this.createHeader();
		this.createItems();

		this.navigator = new Navigator(this.p);
		this.navigator.moveTo(this.items[this.index]);
	}

	private nextItem(): void {
		this.index += 1;

		if (this.index == this.items.length) {
			this.index = 1; // Wrap back around to 1 Skipping 0 because that points to the header
		}

		this.navigator.moveTo(this.items[this.index]);
	}

	private prevItem(): void {
		this.index -= 1;

		if (this.index == 0) {
			this.index = this.items.length - 1; // Wrap back around to the end skipping 0
		}

		this.navigator.moveTo(this.items[this.index]);
	}

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

	private createItems(): void {
		this.builder.setTextSize(ITEM_SIZE);

		MAIN_MENU_ITEM_TEXTS.forEach((itemText) => {
			this.items.push(this.builder.setLabel(itemText).setAbsoluteY(this.currentY).build());
			this.currentY += PIXELS_BETWEEN_ITEMS;
		});
	}

	display(): void {
		if (this.items.length == 0) return;

		this.items.forEach((item) => item.display());
		this.navigator.display();
	}

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
			} else {
				console.error(
					'selected an empty menu item at index: ',
					this.index,
					' available range: ',
					0,
					' ',
					this.items.length - 1
				);
			}
		}

		return '';
	}
}
