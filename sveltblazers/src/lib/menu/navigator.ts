import type p5 from 'p5';
import type { MenuItem } from './menuitem/menu_item';

const DISTANCE_TO_ITEM: number = 20;
const NAVIGATOR_HORIZONTAL_LENGTH: number = 40;
const NAVIGATOR_VERTICAL_LENGTH: number = 20;

/**
 * Represents a navigation cursor for menu selection.
 * This class handles the visual indication of the currently selected menu item.
 */
export class Navigator {
	private p: p5;
	private menuItem: MenuItem | null = null;
	private visible: boolean = true;

	/**
	 * Creates a Navigator instance associated with a p5 drawing context.
	 * @param {p5} p - The p5 instance used for drawing the navigator.
	 */
	constructor(p: p5) {
		this.p = p;
	}

	/**
	 * Updates the navigator to point to a new menu item.
	 * @param {MenuItem} menuItem - The menu item to navigate to.
	 */
	public moveTo(menuItem: MenuItem): void {
		this.menuItem = menuItem;
	}

	/**
	 * Retrieves the currently navigated menu item.
	 * @returns {MenuItem | null} The current menu item, or null if none is selected.
	 */
	public currentItem(): MenuItem | null {
		return this.menuItem;
	}

	/**
	 * Toggles the visibility of the navigator and returns the new visibility state.
	 * @returns {boolean} The new visibility state of the navigator.
	 */
	public toggle(): boolean {
		this.visible = !this.visible;
		return this.visible;
	}

	/**
	 * Displays the navigator on the p5 canvas if it is visible and a menu item is selected.
	 * The navigator is drawn as a triangle pointing towards the selected menu item.
	 */
	public display(): void {
		if (!this.menuItem || !this.visible) {
			return;
		}

		const itemX: number = this.menuItem.getX();
		const itemY: number = this.menuItem.getY();
		const itemSize: number = this.menuItem.getTextSize();
		const leftmost_x = itemX - (DISTANCE_TO_ITEM + NAVIGATOR_HORIZONTAL_LENGTH);

		// Drawing a triangle pointing towards the selected menu item
		this.p.triangle(
			leftmost_x, // Top x point
			itemY - itemSize - NAVIGATOR_VERTICAL_LENGTH / 2, // Top y point
			itemX - DISTANCE_TO_ITEM, // Right x point, directly next to the item
			itemY - itemSize / 2, // Central y point
			leftmost_x, // Bottom x point
			itemY + NAVIGATOR_VERTICAL_LENGTH / 2 // Bottom y point
		);
	}
}
