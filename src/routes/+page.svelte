<script lang="ts">
	import { isLoggedIn } from '../stores/auth';
	import Welcome from '../components/Welcome.svelte';
	import Login from '../components/Login.svelte';

</script>

<div class="h-full w-full flex justify-center items-center p-4">
	{#if $isLoggedIn}
		<Welcome />
	{:else}
		<div class="w-full max-w-md">
			<Login />
		</div>
	{/if}
</div>
