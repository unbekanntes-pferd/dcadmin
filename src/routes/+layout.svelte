<script lang="ts">
	import '../app.postcss';

	// Floating UI for Popups
	import { computePosition, autoUpdate, flip, shift, offset, arrow } from '@floating-ui/dom';
	import { AppRail, AppRailAnchor, storePopup } from '@skeletonlabs/skeleton';
	import { isLoggedIn, logout, userAccount } from '../stores/auth';
	import { page } from '$app/stores';
	import PermissionsIcon from '~icons/mdi/account-key';
	import EventsIcon from '~icons/mdi/event-search';
	import LogoutIcon from '~icons/mdi/logout';
	import HomeIcon from '~icons/mdi/home';
	import NodesIcon from '~icons/mdi/folder-account';
	import { goto } from '$app/navigation';
	import { initializeStores, Toast, TabAnchor, TabGroup } from '@skeletonlabs/skeleton';

	initializeStores();
	storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });

	$: if (!$isLoggedIn) {
		goto('/');
	}

	const handleLogout = () => {
		logout();
		goto('/');
	};
</script>

<Toast />

<div class="flex flex-row h-full w-full">
	{#if $isLoggedIn && $userAccount}
		<div class="w-max">
			<AppRail>
				<AppRailAnchor href="/" selected={$page.url.pathname === '/'}>
					<svelte:fragment slot="lead"><HomeIcon /></svelte:fragment>
					Home
				</AppRailAnchor>
				{#if !$userAccount.isCloud}
					<AppRailAnchor href="/events" selected={$page.url.pathname === '/events'}>
						<svelte:fragment slot="lead"><EventsIcon /></svelte:fragment>
						<div class="flex flex-col text-xs">Events</div>
					</AppRailAnchor>
					<AppRailAnchor
						href="/users"
						selected={$page.url.pathname === '/users'}
					>
						<svelte:fragment slot="lead"><PermissionsIcon /></svelte:fragment>
						<div class="flex flex-col text-xs">Users</div>
					</AppRailAnchor>
					<AppRailAnchor
					href="/nodes/0"
					selected={$page.url.pathname.startsWith('/nodes')}
				>
					<svelte:fragment slot="lead"><NodesIcon /></svelte:fragment>
					<div class="flex flex-col text-xs">Nodes</div>
				</AppRailAnchor>
				{/if}
				<svelte:fragment slot="trail">
					<AppRailAnchor on:click={handleLogout}>
						<svelte:fragment slot="lead"><LogoutIcon /></svelte:fragment>
						<div class="flex flex-col text-xs">Logout</div>
					</AppRailAnchor>
				</svelte:fragment>
			</AppRail>
		</div>
	{/if}

	<slot />
</div>
