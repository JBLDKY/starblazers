<script lang="ts">
	import { ListBox, ListBoxItem } from '@skeletonlabs/skeleton';
	let listBoxValue: string = 'account';
</script>

<div class="flex h-screen bg-surface-800">
	<!-- ListBox Container -->
	<div class="left-0 top-0 z-40 flex h-full w-64 overflow-auto bg-surface-600 text-tertiary-500">
		<!-- Example of a ListBox component -->
		<ListBox class="ml-auto mr-auto w-5/6 pt-20">
			<ListBoxItem bind:group={listBoxValue} name="hi" value="account">Account</ListBoxItem>
			<ListBoxItem bind:group={listBoxValue} name="hi" value="privacy">Privacy</ListBoxItem>
			<ListBoxItem bind:group={listBoxValue} name="hi" value="security">Security</ListBoxItem>
			<ListBoxItem bind:group={listBoxValue} name="hi" value="game">Game</ListBoxItem>
			<ListBoxItem bind:group={listBoxValue} name="hi" value="stats">Stats</ListBoxItem>
		</ListBox>
	</div>

	<div class="flex-1 p-8">
		{#if listBoxValue === 'account'}
			<div class="flex-1 space-y-4">
				<label class="flex items-center">
					<span class="block w-1/3 text-sm font-medium text-tertiary-500">Username:</span>
					<input
						type="text"
						class="placeholder-opacity-400 flex-2 w-400 rounded-md bg-tertiary-500 px-3 py-2 text-primary-700 placeholder-primary-600 focus:border-primary-500 focus:bg-tertiary-600 focus:outline-1 focus:outline-secondary-900 focus:ring-primary-500"
						placeholder="<PLAYER USERNAME>"
					/>
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
	</div>
</div>
