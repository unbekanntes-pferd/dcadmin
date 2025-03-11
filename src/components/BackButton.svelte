<script lang="ts">
	import BackIcon from '~icons/mdi/arrow-back-circle';

	export let full = false;
	export let back = () => window.history.back();
</script>

<button on:click={back} class={full ? 'btn variant-outline-primary' : ''}>
	<div class="flex flex-row">
		<span class="mr-2"><BackIcon /></span>
		<span>Back</span>
	</div>
</button>
