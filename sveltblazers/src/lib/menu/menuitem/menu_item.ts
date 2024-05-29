import type p5 from 'p5';
import type { MenuItemBuilder } from './menu_item_builder';

/**
 * Represents a single item within a menu, encapsulating its properties and display method.
 * This class is typically constructed using a MenuItemBuilder instance for better parameter management.
 */
export class MenuItem {
	private p: p5;
	private label: string;
	private x: number;
	private y: number;
	private textSize: number;

	/**
	 * Constructs a MenuItem instance using properties specified in a MenuItemBuilder.
	 * @param {MenuItemBuilder} builder - The builder instance containing all necessary properties.
	 */
	constructor(builder: MenuItemBuilder) {
		this.p = builder.p;
		this.label = builder.label;
		this.x = builder.x;
		this.y = builder.y;
		this.textSize = builder.textSize;
	}

	/**
	 * Gets the label of the menu item.
	 * @returns {string} The label of the menu item.
	 */
	getLabel(): string {
		return this.label;
	}

	/**
	 * Gets the x-coordinate of the menu item's position on the canvas.
	 * @returns {number} The x-coordinate of the menu item.
	 */
	getX(): number {
		return this.x;
	}

	/**
	 * Gets the y-coordinate of the menu item's position on the canvas.
	 * @returns {number} The y-coordinate of the menu item.
	 */
	getY(): number {
		return this.y;
	}

	/**
	 * Gets the text size used for the menu item.
	 * @returns {number} The size of the text.
	 */
	getTextSize(): number {
		return this.textSize;
	}

	/**
	 * Displays the menu item on the p5 canvas.
	 * This method sets the text size and draws the label at the specified coordinates.
	 */
	display(): void {
		this.p.textSize(this.textSize);
		this.p.text(this.label, this.x, this.y);
	}
}
