import type p5 from 'p5';
import { Navigator } from './navigator';
import type { MenuItem } from './menuitem/menu_item';
import type { InputHandler } from '$lib/system/input_handler';

/**
 * Abstract class representing the base structure of a menu in a p5 application.
 * This class provides a framework for menu functionalities, including display and input handling.
 */
export abstract class BaseMenu {
	protected p: p5;
	protected items: MenuItem[] = [];
	protected navigator: Navigator;
	protected index: number = 1; // Assuming 0 can be a default start index
	protected inputHandler: InputHandler;

	/**
	 * Constructs a base menu.
	 * @param {p5} p - The p5 instance on which the menu will operate.
	 */
	constructor(p: p5, inputHandler: InputHandler) {
		this.p = p;
		this.navigator = new Navigator(this.p);
		this.inputHandler = inputHandler;
	}

	/**
	 * Selects the next MenuItem.
	 */
	protected nextItem(): void {
		this.index += 1;

		if (this.index == this.items.length) {
			this.index = 1; // Wrap back around to 1 Skipping 0 because that points to the header
		}

		this.navigator.moveTo(this.items[this.index]);
	}

	/**
	 * Selects the previous MenuItem.
	 */
	protected prevItem(): void {
		this.index -= 1;

		if (this.index == 0) {
			this.index = this.items.length - 1; // Wrap back around to the end skipping 0
		}

		this.navigator.moveTo(this.items[this.index]);
	}

	/**
	 * Displays the menu by looping over the items and calling display for each. Also displays the navigator.
	 */
	display(): void {
		if (this.items.length === 0) return;
		this.items.forEach((item) => item.display());
		this.navigator.display();
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

		if (cachedKeyPresses['Escape']) {
			cachedKeyPresses['Escape'] = false;
			return 'Main menu';
		}

		if (cachedKeyPresses['Enter']) {
			const selected = this.navigator.currentItem();
			cachedKeyPresses['Enter'] = false;
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

	abstract loop: (timestamp: number) => void;
}
