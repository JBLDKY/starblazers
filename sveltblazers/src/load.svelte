<script context="module" lang="ts">
	import { redirect } from '@sveltejs/kit';

	// The `load` function is automatically invoked by SvelteKit with the context parameter
	export async function load(isAuthenticated: boolean) {
		// Assume `isAuthenticated` is a property on the session object
		if (!isAuthenticated) {
			throw redirect(302, '/login');
		}

		// Return props or perform other actions if authenticated
		return {
			props: {
				// Your props here
			}
		};
	}
</script>
