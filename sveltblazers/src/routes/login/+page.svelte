<script lang="ts">
	import LoginForm from './loginForm.svelte';
	import { writable, get } from 'svelte/store';
	import { goto } from '$app/navigation';
	import { ProgressRadial, getToastStore } from '@skeletonlabs/skeleton';
	import { jwtStore } from '../../store/auth';
	import {
		loginMessages,
		AUTH_SERVER_URL,
		CREATE_NEW_SERVER_URL,
		LOGIN_DELAY
	} from '../../constants';
	import { form_div_wrapper_tw, button_tw, result_text } from '../../tailwind_presets';
	import RegisterForm from './registerForm.svelte';
	import { validateJwt } from '../../hooks/withJwt';
	import { onMount } from 'svelte';

	const toastStore = getToastStore();
	onMount(async () => {
		// This is a protected page; login is required
		// If this is not inside onMount(), it will raise an error that
		// `goto()` cannot be called on the server
		if (get(jwtStore) === undefined || get(jwtStore) == '') {
			toastStore.trigger({ message: 'You are not logged in!' });
			goto('/login');
		} else {
			try {
				await validateJwt();
				goto('/account');
			} catch (error) {
				console.log('Re-directing to log in because token is not valid');
			}
		}
	});

	async function test() {
		try {
			const response = await fetch('http://localhost:3030/helloworld', {
				method: 'GET',
				credentials: 'include'
			});
			if (response.ok) {
				const data = await response.json();
				console.log('Protected data:', data);
			} else {
				console.error('Failed to fetch protected data:', response.status);
			}
		} catch (error) {
			console.error('Error fetching data:', error);
		}
	}

	let randomFunnyMessage = loginMessages[Math.floor(Math.random() * loginMessages.length)];
	function updateMessage() {
		randomFunnyMessage = loginMessages[Math.floor(Math.random() * loginMessages.length)];
	}

	let showLoginForm = writable(true);

	let toggleForm = () => {
		showLoginForm.update((value) => !value);
	};

	let email: string = '';
	let username: string = '';
	let password: string = '';
	let result;

	function delay(ms = LOGIN_DELAY) {
		return new Promise((resolve) => setTimeout(resolve, ms));
	}

	async function createNewAccount() {
		const body = { username: username, email: email, password: password };
		console.log('body: ', body);
		let response = await fetch(CREATE_NEW_SERVER_URL, {
			method: 'POST',
			mode: 'cors', // no-cors, *cors, same-origin,
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(body)
		});
		// Give user time to read funni msg
		await delay(LOGIN_DELAY);
		let text = await response.json();
		return text;
	}

	async function authenticate() {
		const body = { email: email, password: password };
		console.log(body);
		let response = await fetch(AUTH_SERVER_URL, {
			method: 'POST',
			mode: 'cors',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(body)
		});
		// Give user time to read funni msg
		await delay(LOGIN_DELAY);

		const jwt = response.headers.get('Authorization');
		if (jwt === undefined || jwt === null) {
			throw new Error('Did not receive jwt from server');
		}
		jwtStore.set(jwt);

		// Upon succesful login, redirect to the account page
		goto('/account');
	}

	function submitHandler() {
		updateMessage();

		if (get(showLoginForm)) {
			result = authenticate();
		} else {
			result = createNewAccount();
		}

		// Clear out the data fields
		email = '';
		password = '';
	}
</script>

<div class="bg-surface-800">
	<button on:click={test} class={button_tw}>Test</button>
	<div class={form_div_wrapper_tw}>
		{#if result === undefined}
			{#if $showLoginForm}
				<LoginForm bind:email bind:password bind:toggleForm onSubmit={submitHandler} />
			{:else}
				<RegisterForm
					bind:username
					bind:email
					bind:password
					bind:toggleForm
					onSubmit={submitHandler}
				/>
			{/if}
		{:else}
			{#await result}
				<div class="ml-1/2 mr-1/2 absolute mb-40">
					<ProgressRadial background="pink" class="color-secondary-900" value={undefined} />
				</div>
				<div class={result_text}><span>{randomFunnyMessage}</span></div>
			{:then value}
				{#if $showLoginForm}
					<div class={result_text}><span>{value}</span></div>
				{:else}
					<div class={result_text}>
						<span>Welcome {username}, your ship is being readied.</span>
					</div>
				{/if}
			{:catch error}
				<div class={result_text}>
					<span>Could not authenticate: {error.message} </span>
				</div>
			{/await}
		{/if}
	</div>
</div>
