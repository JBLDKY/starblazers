import type p5 from 'p5';
import type { MenuItem } from './menuitem/menu_item';

const DISTANCE_TO_ITEM: number = 20;
const NAVIGATOR_HORIZONTAL_LENGTH: number = 40;
const NAVIGATOR_VERTICAL_LENGTH: number = 20;

export class Navigator {
	private p: p5;
	private menuItem: MenuItem | null = null;
	private visible: boolean = true;

	constructor(p: p5) {
		this.p = p;
	}

	public moveTo(menuItem: MenuItem): void {
		this.menuItem = menuItem;
	}

	public currentItem(): MenuItem | null {
		return this.menuItem;
	}

	public toggle(): boolean {
		this.visible = !this.visible;
		return this.visible;
	}

	public display(): void {
		if (!this.menuItem || !this.visible) {
			return;
		}

		const itemX: number = this.menuItem.getX();
		const itemY: number = this.menuItem.getY();
		const itemSize: number = this.menuItem.getTextSize();
		const leftmost_x = DISTANCE_TO_ITEM + NAVIGATOR_HORIZONTAL_LENGTH;

		this.p.triangle(
			itemX - leftmost_x, // Top x point
			itemY - itemSize - NAVIGATOR_VERTICAL_LENGTH / 2, // Top x point
			itemX - DISTANCE_TO_ITEM, // Point y on the right
			itemY - itemSize / 2, // Point y on the right
			itemX - leftmost_x, // Bottom x point
			itemY + NAVIGATOR_VERTICAL_LENGTH / 2 // Bottom y point
		);
	}
}
