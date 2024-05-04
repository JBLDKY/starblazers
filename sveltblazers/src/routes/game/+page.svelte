<script lang="ts">
	import P5, { type Sketch } from 'p5-svelte';
	import { onMount } from 'svelte';
	import { SpaceInvadersGame } from '../../lib/game/game';
	import { jwtStore } from '../../store/auth';
	import { get } from 'svelte/store';
	import { goto } from '$app/navigation';
	import { getToastStore } from '@skeletonlabs/skeleton';

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

	const sketch: Sketch = (p5) => {
		p5.setup = () => {
			p5.createCanvas(1280, 800);
			const spaceInvadersGame: SpaceInvadersGame = new SpaceInvadersGame(p5);
			spaceInvadersGame.start();
		};

		p5.draw = () => {
			spaceInvadersGame.update();
			spaceInvadersGame.draw();
		};
	};
</script>

<div class="game m-0 flex h-screen w-screen flex-col items-center justify-center bg-[#221569] p-0">
	<P5 {sketch} />
</div>
