<script>
	import { TabGroup } from '@skeletonlabs/skeleton';
	import { TabAnchor } from '@skeletonlabs/skeleton';
	import { jwtStore } from '../store/auth';

	import '../app.pcss';

	const logout = () => {
		jwtStore.set('');
	};
</script>

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
<slot></slot>
