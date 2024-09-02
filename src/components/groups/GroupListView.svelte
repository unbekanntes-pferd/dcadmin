<script lang="ts">
	import type { GroupList } from '$lib/groups/models';
	import { formatUTCDateTimeShort } from '$lib/utils';
	import SettingsIcon from '~icons/mdi/settings';
	import RoomsConfigIcon from '~icons/mdi/folder-cog';
	import UserIcon from '~icons/mdi/user';
	import GroupIcon from '~icons/mdi/account-group';
	import AuditorIcon from '~icons/mdi/event-search';
	import UsersIcon from '~icons/mdi/users';
	import { roleListForGroups } from '$lib/users/models';
	import { goto } from '$app/navigation';

	export let groupList: GroupList;
	$: groups = groupList.items;

	const isConfigManager = (idx: number): boolean => {
		// either no groups / or no roles - bail
		console.log(idx);
		if (idx > groups.length - 1 || !groups[idx].groupRoles || groups[idx].groupRoles.items.length === 0) {
			console.log(`bailing out for ${idx}`);
			return false;
		}

		return groups[idx].groupRoles?.items.some((role) => role.name === roleListForGroups[0].value);
	};

	const isRoomManager = (idx: number): boolean => {
		// either no groups / or no roles - bail
		if (idx > groups.length - 1 || !groups[idx].groupRoles || groups[idx].groupRoles.items.length === 0) {
			return false;
		}

		return groups[idx].groupRoles?.items.some((role) => role.name === roleListForGroups[1].value);
	};

	const isUserManager = (idx: number): boolean => {
		// either no groups / or no roles - bail
		if (idx > groups.length - 1 || !groups[idx].groupRoles || groups[idx].groupRoles.items.length === 0) {
			return false;
		}

		return groups[idx].groupRoles?.items.some((role) => role.name === roleListForGroups[2].value);
	};

	const isGroupManager = (idx: number): boolean => {
		// either no groups / or no roles - bail
		if (idx > groups.length - 1 || !groups[idx].groupRoles || groups[idx].groupRoles.items.length === 0) {
			return false;
		}

		return groups[idx].groupRoles?.items.some((role) => role.name === roleListForGroups[3].value);
	};

	const isAuditor = (idx: number): boolean => {
		// either no groups / or no roles - bail
		if (idx > groups.length - 1 || !groups[idx].groupRoles || groups[idx].groupRoles.items.length === 0) {
			return false;
		}

		return groups[idx].groupRoles?.items.some((role) => role.name === roleListForGroups[4].value);
	};

	const getName = (name: string | undefined): string => {

		//TODO: fix in backend 
		if (name === ' ') {
			return 'SYSTEM_USER';
		}
		return name || 'SYSTEM_USER';
	}

</script>

<div class="table-container h-full overflow-y-auto">
	<table class="table table-hover">
		<thead>
			<tr>
				<th>Name</th>
				<th>Created</th>
				<th>Updated</th>
				<th>Roles</th>
				<th>Users</th>
			</tr>
		</thead>
		<tbody>
			{#each groups as row, idx}
				<tr>
					<td>
						<span>
							{row.name}
						</span>
					</td>
					<td>
						{formatUTCDateTimeShort(row.createdAt)} by {getName(row.createdByName)}
					</td>
					<td>
						{#if row.updatedAt}
							{formatUTCDateTimeShort(row.updatedAt)} by {getName(row.updatedByName)}
						{:else}
							never
						{/if}
					</td>
					<td class="min-w-fit">
						{#if row.groupRoles && row.groupRoles.items.length > 0}
							<div class="flex flex-row">
								{#if isConfigManager(idx)}
									<div>
										<SettingsIcon class="text-green-300 mx-1" />
									</div>
								{/if}
								{#if isRoomManager(idx)}
									<div>
										<RoomsConfigIcon class="text-green-300 mx-1" />
									</div>
								{/if}
								{#if isUserManager(idx)}
									<div>
										<UserIcon class="text-green-300 mx-1" />
									</div>
								{/if}
								{#if isGroupManager(idx)}
									<div><GroupIcon class="text-green-300 mx-1" /></div>
								{/if}
								{#if isAuditor(idx)}
									<div>
										<AuditorIcon class="text-green-300 mx-1" />
									</div>
								{/if}
							</div>
						{:else}
							-
						{/if}
					</td>
					<td>
						{#if row.cntUsers && row.cntUsers > 0}
						<button on:click={() => goto(`/groups/${row.id}`)}>
							<span><UsersIcon /></span>
							<span>
								{row.cntUsers}
							</span>
						</button>
						{:else}
							no users
						{/if}
				</tr>
			{/each}
		</tbody>
	</table>
</div>

<style lang="css">
	td {
		padding: 1rem;
	}

	table thead {
		position: sticky;
		top: 0;
		z-index: 1;
	}
</style>
