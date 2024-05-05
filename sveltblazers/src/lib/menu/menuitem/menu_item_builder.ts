import type p5 from 'p5';
import { MenuItem } from './menu_item';

export class MenuItemBuilder {
	public readonly p: p5;
	public x: number;
	public y: number;
	public label: string = '';
	public textSize: number = 36;

	constructor(p: p5) {
		this.p = p;
		this.x = this.p.width;
		this.y = this.p.height;
		this.p.textSize(this.textSize);
	}

	setLabel(label: string): MenuItemBuilder {
		this.label = label;
		return this;
	}

	setPosition(x: number, y: number): MenuItemBuilder {
		this.x = x;
		this.y = y;
		return this;
	}

	setAbsoluteX(x: number): MenuItemBuilder {
		this.x = x;
		return this;
	}

	setAbsoluteY(y: number): MenuItemBuilder {
		this.y = y;
		return this;
	}

	setRelativeX(float: number): MenuItemBuilder {
		this.x = (this.p.width - this.p.textWidth(this.label)) / (1 / float);
		return this;
	}

	setRelativeY(float: number): MenuItemBuilder {
		this.y = float * this.p.height;
		return this;
	}

	setTextSize(textSize: number): MenuItemBuilder {
		this.textSize = textSize;
		return this;
	}

	build(): MenuItem {
		return new MenuItem(this);
	}

	setFill(fill: string): MenuItemBuilder {
		this.p.fill(fill);
		return this;
	}
}
