<script lang="ts">
	import { TabGroup, Tab } from '@skeletonlabs/skeleton';
	import UserPermissions from '../../components/users/UserPermissions.svelte';
	import ListIcon from '~icons/mdi/format-list-bulleted';
	import PermissionsIcon from '~icons/mdi/user-key';
	import UserList from '../../components/users/UserList.svelte';
	import { lastUserTab } from '../../stores/users';
	import { userAccount } from '../../stores/auth';
	import { cloudCustomerFeatureRestrictionsEnabled } from '$lib/utils';

	let tabSet = $lastUserTab;

	$: $lastUserTab = tabSet;
</script>

<TabGroup class="w-full">
	{#if $userAccount && (!$userAccount.isCloud || !cloudCustomerFeatureRestrictionsEnabled)}
		<Tab bind:group={tabSet} name="permissions" value={0}>
			<svelte:fragment slot="lead"><PermissionsIcon /></svelte:fragment>
			<span>Permissions</span>
		</Tab>
	{/if}

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
