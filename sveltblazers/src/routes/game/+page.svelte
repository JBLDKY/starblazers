<script lang="ts">
	import P5, { type Sketch } from 'p5-svelte';
	import { onMount } from 'svelte';
	import { SpaceInvadersGame } from '../../lib/game/game';
	import { jwtStore } from '../../store/auth';
	import { get } from 'svelte/store';
	import { goto } from '$app/navigation';
	import { getToastStore } from '@skeletonlabs/skeleton';
	import ChatBox from './ChatBox.svelte';

	const toastStore = getToastStore();
	let spaceInvadersGame: SpaceInvadersGame;

	onMount(() => {
		// This is a protected page; login is required
		// If this is not inside onMount(), it will raise an error that
		// `goto()` cannot be called on the server.
		if (get(jwtStore) === undefined || get(jwtStore) == '') {
			toastStore.trigger({ message: 'You are not logged in!' });
			goto('/login');
		}
	});

	const sketch: Sketch = (p) => {
		p.setup = () => {
			p.createCanvas(1280, 800);

			p.loadFont('/fonts/pressStart2P.ttf', (font) => {
				p.fill('deeppink');
				p.textFont(font);

				// Wait for our font to load before starting the game, else the main menu will not be centered
				const spaceInvadersGame: SpaceInvadersGame = new SpaceInvadersGame(p);
				spaceInvadersGame.start();
			});
		};

		p.draw = () => {
			if (spaceInvadersGame !== undefined) {
				spaceInvadersGame.update();
				spaceInvadersGame.draw();
			}
		};
	};
</script>

<div class="game m-0 flex h-screen w-screen flex-col items-center justify-center bg-black p-0">
	<P5 {sketch} />
	<ChatBox />
</div>
