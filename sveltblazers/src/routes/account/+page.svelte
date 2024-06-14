<script lang="ts">
	import { onMount } from 'svelte';
	import { ListBox, ListBoxItem } from '@skeletonlabs/skeleton';
	import { get } from 'svelte/store';
	import { goto } from '$app/navigation';
	import { validateJwt } from '../../hooks/withJwt';
	import { getToastStore } from '@skeletonlabs/skeleton';
	import { jwtStore } from '../../store/auth';
	import { account_page_input_field, button_tw } from '../../tailwind_presets';

	let ws: WebSocket;

	interface PublicPlayerData {
		authority: string;
		email: string;
		username: string;
		uuid: string;
	}

	const toastStore = getToastStore();
	let player_info: Promise<PublicPlayerData> = get_player_info();

	onMount(async () => {
		ws = new WebSocket('ws://localhost:3030/lobby');

		ws.onopen = () => {
			const jwt = get(jwtStore);
			console.log('lobby connection established');
			ws.send(JSON.stringify({ type: 'auth', jwt: jwt }));
		};

		ws.onmessage = (event) => {
			console.log('received: ', event);
		};

		ws.onclose = (event) => {
			console.log('WebSocket connection closed', event.code, event.reason);
		};

		ws.onerror = (error) => {
			console.error('WebSocket error', error);
		};
		// This is a protected page; login is required
		// If this is not inside onMount(), it will raise an error that
		// `goto()` cannot be called on the server
		if (get(jwtStore) === undefined || get(jwtStore) == '') {
			toastStore.trigger({ message: 'You are not logged in!' });
			goto('/login');
		} else {
			try {
				await validateJwt();
				console.log('JWT is valid');
			} catch (error) {
				console.error('Error checking JWT:', error);
				toastStore.trigger({ message: 'Session expired' });
				goto('/login');
			}
		}
	});

	async function get_player_info(): Promise<PublicPlayerData> {
		const res = await fetch('http://localhost:3030/players/player', {
			method: 'GET',
			headers: {
				authorization: get(jwtStore)
			}
		});

		if (res.ok) {
			return res.json();
		} else {
			throw new Error(await res.text());
		}
	}

	let listBoxValue: string = 'account';

	const chat = () => {
		ws.send(JSON.stringify({ type: 'gamestate' }));
	};
</script>

<div class="flex h-screen bg-surface-800">
	<div class="left-0 top-0 z-40 flex h-full w-64 overflow-auto bg-surface-600 text-tertiary-500">
		<!-- Just placeholders for future functionality-->
		<ListBox class="ml-auto mr-auto w-5/6 pt-20">
			<ListBoxItem bind:group={listBoxValue} name="hi" value="account">Account</ListBoxItem>
			<ListBoxItem bind:group={listBoxValue} name="hi" value="privacy">Privacy</ListBoxItem>
			<ListBoxItem bind:group={listBoxValue} name="hi" value="security">Security</ListBoxItem>
			<ListBoxItem bind:group={listBoxValue} name="hi" value="game">Game</ListBoxItem>
			<ListBoxItem bind:group={listBoxValue} name="hi" value="stats">Stats</ListBoxItem>
		</ListBox>
	</div>

	<button on:click={chat} class={button_tw}>Test</button>
	<button />

	<div class="flex-1 p-8">
		{#await player_info}
			<div class="ml-1/2 mr-1/2 absolute mb-40"></div>
		{:then player_info}
			{#if listBoxValue === 'account'}
				<div class="flex-1 space-y-4">
					<label class="flex items-center">
						<span class="block w-1/3 text-sm font-medium text-tertiary-500">Username: </span>
						<input type="text" class={account_page_input_field} value={player_info['username']} />
					</label>
					<label class="flex items-center">
						<span class="block w-1/3 text-sm font-medium text-tertiary-500">Authority: </span>
						<input type="text" class={account_page_input_field} value={player_info['authority']} />
					</label>
					<label class="flex items-center">
						<span class="block w-1/3 text-sm font-medium text-tertiary-500">Email: </span>
						<input type="text" class={account_page_input_field} value={player_info['email']} />
					</label>
					<label class="flex items-center">
						<span class="block w-1/3 text-sm font-medium text-tertiary-500">uuid: </span>
						<input type="text" class={account_page_input_field} value={player_info['uuid']} />
					</label>
				</div>
			{:else if listBoxValue === 'privacy'}
				<div>Privacy Settings</div>
			{:else if listBoxValue === 'security'}
				<div>Security Settings</div>
			{:else if listBoxValue === 'game'}
				<div>Game Settings</div>
			{:else if listBoxValue === 'stats'}
				<div>Stats</div>
			{/if}
		{:catch error}
			<div>
				<span>Could not authenticate: {error.message} </span>
			</div>
		{/await}
	</div>
</div>
