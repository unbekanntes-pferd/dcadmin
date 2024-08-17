<script lang="ts">
	import {
		roleList,
		type RoleItem,
		type RoleList,
		type UserItem,
		type UserList
	} from '$lib/users/models';
	import { formatUTCDateTime } from '$lib/utils';
	import type { PopupSettings } from '@skeletonlabs/skeleton';

	export let userList: UserList;
	$: users = userList.items;
</script>

<div class="table-container h-full overflow-y-auto">
	<table class="table table-hover">
		<thead>
			<tr>
				<th>Name</th>
				<th>E-Mail</th>
				<th>Login</th>
				<th>Roles</th>
				<th>Last login</th>
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
					</td>
					<td>{row.email}</td>
					<td>{row.userName}</td>
					<td>{row.userRoles ? row.userRoles.items.length : 0}</td>
					<td>
						{row.lastLogin ? formatUTCDateTime(row.lastLogin) : 'never'}
					</td>
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
