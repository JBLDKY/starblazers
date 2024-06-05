export const loginMessages: string[] = [
	'Negotiating peace with aliens... Please stand by.',
	'Warming up the laser cannons... Hold tight, cadet!',
	"Launching in T-minus 10 seconds... Just kidding, we're still loading.",
	'Hitching a ride on the nearest comet... Hang on!',
	'Assembling crew for intergalactic mission... Credentials needed!',
	'Decrypting alien transmissions... Logging you in!',
	"Calibrating photon beams... Don't look directly into the light!",
	'Scanning for space pirates... Secure your belongings!',
	'Configuring gravity generators... Watch your step!',
	'Plotting jump to hyperspace... Credentials confirmed, captain!'
];

const BASE_URL: string = 'http://localhost:3030';
export const AUTH_SERVER_URL: string = `${BASE_URL}/auth/login`;
export const CREATE_NEW_SERVER_URL: string = `${BASE_URL}/players/create`;
export const TEST_ENDPOINT_SERVER_URL: string = `${BASE_URL}/test`;
export const VERIFY_JWT_SERVER_URL: string = `${BASE_URL}/auth/verify_jwt`;

export const LOGIN_DELAY: number = 2000;
export const MAX_BULLETS = 50;

export enum GameState {
	MENU,
	RUN,
	PAUSE
}
