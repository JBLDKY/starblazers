<script>
	import '../app.pcss';
	import { Toast } from '@skeletonlabs/skeleton';
	import { TabGroup } from '@skeletonlabs/skeleton';
	import { TabAnchor } from '@skeletonlabs/skeleton';
	import { jwtStore } from '../store/auth';
	import { initializeStores } from '@skeletonlabs/skeleton';
	import { goto } from '$app/navigation';

	initializeStores();

	const logout = () => {
		// This constitutes `logging out` for the time being
		jwtStore.set('');

		// Redirect the user to the login page
		goto('/login');
	};
</script>

<div>
	<Toast />
	<TabGroup
		justify="justify-center"
		active="variant-filled-primary"
		hover="hover:variant-soft-primary"
		flex="flex-1 lg:flex-none"
		rounded=""
		border=""
		class="w-full bg-surface-600"
	>
		<TabAnchor class="text-tertiary-500" href="/login">
			<svelte:fragment slot="lead">login</svelte:fragment>
		</TabAnchor>
		<TabAnchor class="text-tertiary-500" href="/account">
			<svelte:fragment slot="lead">account</svelte:fragment>
		</TabAnchor>
		<TabAnchor class="text-tertiary-500" href="/game">
			<svelte:fragment slot="lead">game</svelte:fragment>
		</TabAnchor>
		<TabAnchor on:click={logout} class="text-tertiary-500">
			<svelte:fragment slot="lead">logout</svelte:fragment>
		</TabAnchor>
	</TabGroup>
</div>

<slot></slot>
