<script lang="ts">
	import { type UserList } from '$lib/users/models';
	import { formatUTCDateTime, handleUserNavigation } from '$lib/utils';
	import RoomIcon from '~icons/mdi/folder-account';

	export let userList: UserList;
	$: users = userList.items;
</script>

<div class="table-container">
	<table class="table table-hover min-w-full">
		<thead>
			<tr>
				<th>Name</th>
				<th class="hidden sm:table-cell">E-Mail</th>
				<th class="hidden md:table-cell">Login</th>
				<th class="hidden lg:table-cell">Roles</th>
				<th>Last login</th>
				<th>Last admin rooms</th>
			</tr>
		</thead>
		<tbody>
			{#each users as row}
				<tr>
					<td>
						<span>
							{row.firstName}
							{row.lastName}
						</span>
						<span>
							{row.isLocked ? '🔒' : ''}
						</span>
						<div class="sm:hidden text-xs mt-1 text-surface-400">
							<span class="font-semibold">E-Mail:</span>
							{row.email}
						</div>
						<div class="md:hidden text-xs mt-1 text-surface-400">
							<span class="font-semibold">Login:</span>
							{row.userName}
						</div>
						<div class="lg:hidden text-xs mt-1 text-surface-400">
							<span class="font-semibold">Roles:</span>
							{row.userRoles ? row.userRoles.items.length : 0}
						</div>
					</td>
					<td class="hidden sm:table-cell">{row.email}</td>
					<td class="hidden md:table-cell">{row.userName}</td>
					<td class="hidden lg:table-cell">{row.userRoles ? row.userRoles.items.length : 0}</td>
					<td>
						{row.lastLogin ? formatUTCDateTime(row.lastLogin) : 'never'}
					</td>
					<td>
						<div class="text-xs mt-1 text-surface-400">
							<button on:click={() => handleUserNavigation(row.id)}>
								<span><RoomIcon /></span>
							</button>
						</div></td
					>
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
