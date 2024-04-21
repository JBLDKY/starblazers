<script lang="ts">
	import {
		input_field_tw,
		input_label_tw,
		input_field_div_wrapper_tw,
		form_tw,
		button_tw,
		button_spacer,
		link_tw,
		title_text_tw
	} from '../../tailwind_presets';
	export let email: string;
	export let password: string;
	export let username: string;
	export let toggleForm: () => void;
	export let onSubmit: () => void;
</script>

<form on:submit|preventDefault={onSubmit} class={form_tw}>
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
