import type p5 from 'p5';
import { MenuItem } from './menu_item';

/**
 * A builder class for creating MenuItem instances with specified configurations.
 * This class provides a fluent interface for setting properties of a MenuItem.
 */
export class MenuItemBuilder {
	public readonly p: p5;
	public x: number;
	public y: number;
	public label: string = '';
	public textSize: number = 36;

	/**
	 * Constructs a MenuItemBuilder with a reference to a p5 instance.
	 * Initializes the builder with default position at the bottom-right corner of the canvas.
	 * @param {p5} p - The p5 instance used for drawing and measurements.
	 */
	constructor(p: p5) {
		this.p = p;
		this.x = this.p.width;
		this.y = this.p.height;
		this.p.textSize(this.textSize);
	}

	/**
	 * Sets the label of the MenuItem to be built.
	 * @param {string} label - The text label for the MenuItem.
	 * @returns {MenuItemBuilder} This builder instance to allow for method chaining.
	 */
	setLabel(label: string): MenuItemBuilder {
		this.label = label;
		return this;
	}

	/**
	 * Sets the position of the MenuItem using absolute coordinates.
	 * @param {number} x - The absolute x-coordinate for the MenuItem.
	 * @param {number} y - The absolute y-coordinate for the MenuItem.
	 * @returns {MenuItemBuilder} This builder instance to allow for method chaining.
	 */
	setPosition(x: number, y: number): MenuItemBuilder {
		this.x = x;
		this.y = y;
		return this;
	}

	/**
	 * Sets the absolute x-coordinate of the MenuItem.
	 * @param {number} x - The absolute x-coordinate for the MenuItem.
	 * @returns {MenuItemBuilder} This builder instance to allow for method chaining.
	 */
	setAbsoluteX(x: number): MenuItemBuilder {
		this.x = x;
		return this;
	}

	/**
	 * Sets the absolute y-coordinate of the MenuItem.
	 * @param {number} y - The absolute y-coordinate for the MenuItem.
	 * @returns {MenuItemBuilder} This builder instance to allow for method chaining.
	 */
	setAbsoluteY(y: number): MenuItemBuilder {
		this.y = y;
		return this;
	}

	/**
	 * Sets the x-coordinate of the MenuItem relative to the width of the canvas.
	 * @param {number} float - The fraction of the canvas width where the MenuItem should be positioned.
	 * @returns {MenuItemBuilder} This builder instance to allow for method chaining.
	 */
	setRelativeX(float: number): MenuItemBuilder {
		this.x = (this.p.width - this.p.textWidth(this.label)) / (1 / float);
		return this;
	}

	/**
	 * Sets the y-coordinate of the MenuItem relative to the height of the canvas.
	 * @param {number} float - The fraction of the canvas height where the MenuItem should be positioned.
	 * @returns {MenuItemBuilder} This builder instance to allow for method chaining.
	 */
	setRelativeY(float: number): MenuItemBuilder {
		this.y = float * this.p.height;
		return this;
	}

	/**
	 * Sets the text size for the MenuItem.
	 * @param {number} textSize - The text size for the MenuItem.
	 * @returns {MenuItemBuilder} This builder instance to allow for method chaining.
	 */
	setTextSize(textSize: number): MenuItemBuilder {
		this.textSize = textSize;
		return this;
	}

	/**
	 * Finalizes and returns a new MenuItem instance with the configured properties.
	 * @returns {MenuItem} A newly created MenuItem instance.
	 */
	build(): MenuItem {
		return new MenuItem(this);
	}

	/**
	 * Sets the fill color for the text of the MenuItem.
	 * @param {string} fill - The color to fill the text.
	 * @returns {MenuItemBuilder} This builder instance to allow for method chaining.
	 */
	setFill(fill: string): MenuItemBuilder {
		this.p.fill(fill);
		return this;
	}
}
