<script lang="ts">
	import { onMount } from 'svelte';
	import { SpaceInvadersGame } from '../../lib/game/game';
	// import ChatBox from '../../components/ChatBox.svelte';

	let canvasElement: HTMLCanvasElement;

	onMount(() => {
		if (canvasElement) {
			canvasElement.width = 1280;
			canvasElement.height = 800;

			const game: SpaceInvadersGame = new SpaceInvadersGame(canvasElement);
			game.start();
		}
	});
</script>

<div class="game m-0 flex h-screen w-screen flex-col items-center justify-center bg-[#221569] p-0">
	<canvas bind:this={canvasElement} class="bg-[#221569]" id="game-canvas"></canvas>
</div>
