<script lang="ts">
	import { session } from '../auth/session';
	import { writable, get } from 'svelte/store';
	const loginMessages = [
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

	let randomFunnyMessage = loginMessages[Math.floor(Math.random() * loginMessages.length)];
	function updateMessage() {
		randomFunnyMessage = loginMessages[Math.floor(Math.random() * loginMessages.length)];
	}

	let showLoginForm = writable(true);

	let toggleForm = () => {
		showLoginForm.update((value) => !value);
	};

	const AUTH_SERVER_URL = 'http://localhost:3030/auth/login';
	const CREATE_NEW_SERVER_URL = 'http://localhost:3030/players/create';
	let email: string = '';
	let username: string = '';
	let password: string = '';
	let jwt: string;
	let result;
	$: if (jwt) {
		$session.jwt = jwt;
	}

	const LOGIN_DELAY = 2000;
	function delay(ms = LOGIN_DELAY) {
		return new Promise((resolve) => setTimeout(resolve, ms));
	}

	async function createNewAccount() {
		const body = { username: username, email: email, password: password };
		console.log('body: ', body);
		let response = await fetch(CREATE_NEW_SERVER_URL, {
			mode: 'cors', // no-cors, *cors, same-origin,
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(body) // body data type must match "Content-Type" header
		});

		await delay(1500); // Adjust delay as needed, here 1.5 seconds for user experience

		console.log('hi');
		let text = await response.json();
		console.log('bye');
		console.log('text: ', text);

		return text;
	}

	async function authenticate() {
		const body = { email: email, password: password };
		let response = await fetch(AUTH_SERVER_URL, {
			method: 'POST',
			mode: 'cors',
			headers: {
				'Content-Type': 'application/json'
			},
			body: JSON.stringify(body) // body data type must match "Content-Type" header
		});

		await delay(1500); // Adjust delay as needed, here 1.5 seconds for user experience

		let text = await response.json();

		jwt = text.message;

		return text;
	}

	function submitHandler() {
		updateMessage();

		if (get(showLoginForm)) {
			// Login
			result = authenticate();
			console.log(result);
		} else {
			// Create new account
			result = createNewAccount();
		}

		// Clear out the data fields
		email = '';
		password = '';
	}

	let input_field_tw =
		'flex-1 w-full rounded-md px-3 py-2 focus:border-primary-500 focus:outline-1 focus:outline-secondary-900 focus:ring-primary-500 bg-tertiary-500 text-primary-700 placeholder-primary-600 placeholder-opacity-400 focus:bg-tertiary-600';
	let input_field_div_wrapper_tw = 'flex items-center justify-between space-x-4';
	let input_label_tw = 'block text-sm font-medium w-1/3 text-tertiary-500';
	let form_tw = 'space-y-4 rounded-lg bg-white p-8 shadow-md w-full max-w-lg bg-surface-500';
	let form_div_wrapper_tw = 'flex min-h-screen items-center justify-center ';
	let button_tw =
		'rounded-lg text-primary-900 bg-tertiary-500 px-4 py-2 hover:bg-tertiary-600 focus:outline-none focus:ring-2 focus:ring-primary-500 focus:ring-opacity-50 ';
	let result_text = 'text-surface-50 text-l font-medium';
	let button_spacer = 'flex justify-between items-center';
	let link_tw = 'text-tertiary-500 hover:text-blue-600 cursor-pointer text-sm font-medium';
	let title_text_tw = 'text-tertiary-500 text-m font-medium';
</script>

<div class="bg-surface-800">
	<div class={form_div_wrapper_tw}>
		<div>
			{#if result === undefined}
				{#if $showLoginForm}
					<form on:submit|preventDefault={submitHandler} class={form_tw}>
						<span class={title_text_tw}>Welcome back to Starblaze.rs!</span>
						<div class={input_field_div_wrapper_tw}>
							<label for="emailField" class={input_label_tw}>
								<span>Email</span>
							</label>
							<input
								id="emailField"
								class={input_field_tw}
								type="text"
								placeholder="Enter your email"
								bind:value={email}
							/>
						</div>
						<div class={input_field_div_wrapper_tw}>
							<label for="passwordField" class={input_label_tw}>
								<span>Password</span>
							</label>
							<input
								id="passwordField"
								class={input_field_tw}
								type="password"
								placeholder="Enter your password"
								bind:value={password}
							/>
						</div>
						<div class={button_spacer}>
							<span class={link_tw} on:click={toggleForm}>New? Click here to sign up!</span>
							<button class={button_tw}> Login </button>
						</div>
					</form>
				{:else}
					<form on:submit|preventDefault={submitHandler} class={form_tw}>
						<span class={title_text_tw}>Start your new adventure!</span>
						<div class={input_field_div_wrapper_tw}>
							<label for="usernameField" class={input_label_tw}>
								<span>Username</span>
							</label>
							<input
								id="usernameField"
								class={input_field_tw}
								type="text"
								placeholder="Pick a username"
								bind:value={username}
							/>
						</div>
						<div class={input_field_div_wrapper_tw}>
							<label for="emailField" class={input_label_tw}>
								<span>Email</span>
							</label>
							<input
								id="emailField"
								class={input_field_tw}
								type="text"
								placeholder="Enter your email"
								bind:value={email}
							/>
						</div>
						<div class={input_field_div_wrapper_tw}>
							<label for="passwordField" class={input_label_tw}>
								<span>Password</span>
							</label>
							<input
								id="passwordField"
								class={input_field_tw}
								type="password"
								placeholder="Enter your password"
								bind:value={password}
							/>
						</div>
						<div class={button_spacer}>
							<span class={link_tw} on:click={toggleForm}>Take me back to login!</span>
							<button class={button_tw}> Create </button>
						</div>
					</form>
				{/if}
			{:else}
				{#await result}
					<div class={result_text}><span>{randomFunnyMessage}</span></div>
				{:then value}
					{#if $showLoginForm}
						<div class={result_text}><span>Json Web Token: {value.body}</span></div>
					{:else}
						<div class={result_text}><span>Welcome: {value.message}</span></div>
					{/if}
				{:catch error}
					<div class={result_text}><span>Could not authenticate: {error.message}</span></div>
				{/await}
			{/if}
		</div>
	</div>
</div>
