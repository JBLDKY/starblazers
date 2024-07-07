<script lang="ts">
	import P5, { type Sketch } from 'p5-svelte';
	import { onDestroy, onMount } from 'svelte';
	import { SpaceInvadersGame } from '../../lib/game/game';
	import { jwtStore, playerInfoStore } from '../../store/auth';
	import { get } from 'svelte/store';
	import { goto } from '$app/navigation';
	import { GameConnection } from '$lib/gcm';
	import { getToastStore } from '@skeletonlabs/skeleton';
	import ChatBox from './ChatBox.svelte';
	import { validateJwt } from '../../hooks/withJwt';
	import { get_player_info } from '../helpers';
	import { LOGIN_DELAY } from '../../constants';

	const toastStore = getToastStore();
	let spaceInvadersGame: SpaceInvadersGame;
	let gameConnection: GameConnection;

	function delay(ms = LOGIN_DELAY) {
		return new Promise((resolve) => setTimeout(resolve, ms));
	}

	onMount(async () => {
		// This is a protected page; login is required
		// If this is not inside onMount(), it will raise an error that
		// `goto()` cannot be called on the server

		if (get(jwtStore) === undefined || get(jwtStore) == '') {
			toastStore.trigger({ message: 'You are not logged in!' });
			goto('/login');
		} else {
			try {
				const player_info = await get_player_info();
				playerInfoStore.set(player_info);
			} catch (error) {
				console.error(error);
				toastStore.trigger({ message: 'Who are you?' });
				goto('/login');
			}
			try {
				await validateJwt();
				console.log('JWT is valid');
			} catch (error) {
				console.error('Error checking JWT:', error);
				toastStore.trigger({ message: 'Session expired' });
				goto('/login');
			}
		}

		if (get(playerInfoStore)) {
			gameConnection = new GameConnection();
			gameConnection.connect();
			await delay(3);
		}
	});

	onDestroy(() => {
		if (gameConnection) {
			gameConnection.disconnect();
		}
	});

	const sketch: Sketch = (p) => {
		p.setup = () => {
			p.createCanvas(1280, 800);
			p.loadFont('/fonts/pressStart2P.ttf', (font) => {
				p.fill('deeppink');
				p.textFont(font);
				const res = get(playerInfoStore);
				// Wait for our font to load before starting the game, else the main menu will not be centered
				spaceInvadersGame = new SpaceInvadersGame(p, res['uuid'], gameConnection);
				spaceInvadersGame.start();

				// Pass the gameConnection to your SpaceInvadersGame if needed
				// spaceInvadersGame.setGameConnection(gameConnection);
			});
		};

		p.draw = () => {
			if (spaceInvadersGame) {
				spaceInvadersGame.update();
			}
		};
	};
</script>

<div class="game m-0 flex h-screen w-screen flex-col items-center justify-center bg-black p-0">
	<!-- {#await get(playerInfoStore)} -->
	<!-- 	<div>connecting ...</div> -->
	<!-- {:then whatever} -->
	<P5 {sketch} />
	<ChatBox />
	<!-- {:catch error} -->
	<!-- 	<div> -->
	<!-- 		<span>Could not authenticate: {error.message} </span> -->
	<!-- 	</div> -->
	<!-- {/await} -->
</div>
