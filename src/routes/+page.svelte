<script lang="ts">
	import { isLoggedIn } from '../stores/auth';
	import Welcome from '../components/Welcome.svelte';
	import Login from '../components/Login.svelte';

</script>

<div class="container h-full w-full flex justify-center items-center">
	{#if $isLoggedIn}
		<Welcome />
	{:else}
		<Login />
	{/if}
</div>
