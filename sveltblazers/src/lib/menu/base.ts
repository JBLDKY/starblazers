import type p5 from 'p5';

export abstract class BaseMenu {
	protected p: p5;

	constructor(p: p5) {
		this.p = p;
	}

	abstract display(): void;

	abstract handleInput(key: { [key: string]: boolean }): string;
}
