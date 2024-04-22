<script lang="ts">
	import ChatInput from './ChatInput.svelte';
</script>

<div class="chat-box fixed bottom-2 left-2 z-10 w-72 bg-[#221569]">
	<div class="chat-messages h-24 overflow-y-hidden break-words bg-[#221569] text-[#b5e6ff]"></div>
	<hr class="chat-line m-0 border-t border-black" />

	<ChatInput />
</div>
