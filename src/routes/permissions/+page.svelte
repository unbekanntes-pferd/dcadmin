<script lang="ts">
	import { Tab, TabAnchor, TabGroup } from '@skeletonlabs/skeleton';
	import UserPermissions from '../../components/UserPermissions.svelte';
	import UsersIcon from '~icons/mdi/users';
	import RoomsIcon from '~icons/mdi/folder-account-outline';
	import { userAccount } from '../../stores/auth';

	let tabSet = 0;
</script>

<div class="w-full">
	<TabGroup>
		{#if $userAccount && $userAccount.isUserManager}
			<Tab bind:group={tabSet} name="users" value={0}>
				<svelte:fragment slot="lead"><UsersIcon /></svelte:fragment>
				<span>Users</span>
			</Tab>
		{/if}
		{#if $userAccount && $userAccount.isRoomManager}
			<Tab bind:group={tabSet} name="nodes" value={1}>
				<svelte:fragment slot="lead"><RoomsIcon /></svelte:fragment>
				<span>Rooms</span>
			</Tab>
		{/if}

		<!-- Tab Panels --->
		<svelte:fragment slot="panel">
			{#if tabSet === 0}
				<UserPermissions />
			{:else if tabSet === 1}
				not implemented
			{/if}
		</svelte:fragment>
	</TabGroup>
</div>
