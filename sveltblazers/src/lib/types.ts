export interface Shape {
	pos: Position;
	dimensions: Dimensions;
}

export class Rectangle implements Shape {
	pos: Position;
	dimensions: Dimensions;

	constructor(pos: Position, dimensions: Dimensions) {
		this.pos = pos;
		this.dimensions = dimensions;
	}
}

export class Circle implements Shape {
	pos: Position;
	dimensions: Dimensions;
	r: number;

	constructor(pos: Position, r: number) {
		this.pos = pos;
		this.dimensions = { width: 2 * r, height: 2 * r };
		this.r = r;
	}
}

export type Dimensions = {
	width: number;
	height: number;
};

export type Position = {
	x: number;
	y: number;
};
