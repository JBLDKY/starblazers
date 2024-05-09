export interface Shape {
	pos: Position;
	dimensions: Dimensions;
}

export type Position = {
	x: number;
	y: number;
};

export class Rectangle implements Shape {
	pos: Position;
	dimensions: Dimensions;

	constructor(pos: Position, dimensions: Dimensions) {
		this.pos = pos;
		this.dimensions = dimensions;
	}
}

export type Dimensions = {
	width: number;
	height: number;
};
export type Circle = {
	pos: Position;
	r: number;
};
