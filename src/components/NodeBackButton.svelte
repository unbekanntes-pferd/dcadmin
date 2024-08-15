<script lang="ts">
	import { handleNodeNavigation } from '$lib/utils';
	import BackIcon from '~icons/mdi/arrow-back-circle';

	export let full = false;
</script>

<button on:click={() => handleNodeNavigation(0)} class={full ? 'btn variant-outline-primary' : ''}>
	<div class="flex flex-row">
		<span class="mr-2"><BackIcon /></span>
		<span>Back</span>
	</div>
</button>
