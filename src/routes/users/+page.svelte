<script lang="ts">
	import { TabGroup, Tab } from '@skeletonlabs/skeleton';
	import UserPermissions from '../../components/users/UserPermissions.svelte';
	import ListIcon from '~icons/mdi/format-list-bulleted';
	import PermissionsIcon from '~icons/mdi/user-key';
	import UserList from '../../components/users/UserList.svelte';

	let tabSet: number = 0;
</script>

<TabGroup class="w-full">
	<Tab bind:group={tabSet} name="permissions" value={0}>
		<svelte:fragment slot="lead"><PermissionsIcon /></svelte:fragment>
		<span>Permissions</span>
	</Tab>
	<Tab bind:group={tabSet} name="userlist" value={1}>
		<svelte:fragment slot="lead"><ListIcon /></svelte:fragment>
		<span>List</span>
	</Tab>

	<!-- Tab Panels --->
	<svelte:fragment slot="panel">
		<div class="flex">
			{#if tabSet === 0}
				<UserPermissions />
			{:else if tabSet === 1}
				<UserList />
			{/if}
		</div>
	</svelte:fragment>
</TabGroup>
