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

export enum Key {
	A = 65,
	B = 66,
	C = 67,
	D = 68,
	E = 69,
	F = 70,
	G = 71,
	H = 72,
	I = 73,
	J = 74,
	K = 75,
	L = 76,
	M = 77,
	N = 78,
	O = 79,
	P = 80,
	Q = 81,
	R = 82,
	S = 83,
	T = 84,
	U = 85,
	V = 86,
	W = 87,
	X = 88,
	Y = 89,
	Z = 90
}
