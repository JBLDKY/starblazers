import type p5 from 'p5';
import type { MenuItemBuilder } from './menu_item_builder';

export class MenuItem {
	private p: p5;
	private label: string;
	private x: number;
	private y: number;
	private textSize: number;

	constructor(builder: MenuItemBuilder) {
		this.p = builder.p;
		this.label = builder.label;
		this.x = builder.x;
		this.y = builder.y;
		this.textSize = builder.textSize;
	}

	getLabel(): string {
		return this.label;
	}

	getX(): number {
		return this.x;
	}

	getY(): number {
		return this.y;
	}

	getTextSize(): number {
		return this.textSize;
	}

	display(): void {
		this.p.textSize(this.textSize);
		this.p.text(this.label, this.x, this.y);
	}
}
