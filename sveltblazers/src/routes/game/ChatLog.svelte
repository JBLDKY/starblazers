<script lang="ts">
	import { chatLogStore } from '../../store/chat';
	import { onDestroy } from 'svelte';

	let messages: string[];

	// Initialization
	const unsubscribe = chatLogStore.subscribe((newMessages) => {
		messages = newMessages;
	});

	// Cleanup
	onDestroy(() => {
		unsubscribe();
	});
</script>

<div class="bg-white">
	{#each messages as msg}
		<div class="bg-surface-500 text-tertiary-500">{msg}</div>
	{/each}
</div>
