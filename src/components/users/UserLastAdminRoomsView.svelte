<script lang="ts">
	import { type LastAdminUserRoomList } from '$lib/users/models';
	import { handleNodeNavigation } from '$lib/utils';

	export let roomList: LastAdminUserRoomList;
	$: rooms = roomList.items;

	import UsersIcon from '~icons/mdi/users';
	import UserBackButton from './UserBackButton.svelte';
</script>

{#if rooms.length > 0}
	<div class="space-y-2">
		<div class="table-container">
			<table class="table table-hover min-w-full">
				<thead>
					<tr>
						<th>Name</th>
						<th class="hidden md:table-cell">Path</th>
						<th class="hidden sm:table-cell">Permissions</th>
						<th class="hidden lg:table-cell">Parent permissions</th>
						<th class="hidden sm:table-cell">Admin in group</th>
					</tr>
				</thead>
				<tbody>
					{#each rooms as room}
						<tr>
							<td>
								<div>{room.name}</div>
							</td>
							<td>
								<div class="md:hidden text-xs mt-1 text-surface-400">
									<span class="font-semibold">Path:</span>
									{room.parentPath}
								</div>
							</td>
							<td>
								<div class="sm:hidden text-xs mt-1 text-surface-400">
									<button on:click={() => handleNodeNavigation(room.id, true)}>
										<span><UsersIcon /></span>
									</button>
								</div>
							</td>

							<td>
								<div class="text-xs mt-1 text-surface-400">
                                    {#if room.parentId}
									<button on:click={() => handleNodeNavigation(room.parentId, true)}>
										<span><UsersIcon /></span>
									</button>
                                    {:else}
                                    -
                                    {/if}
								</div>
							</td>
							<td>
								<div class="sm:hidden text-xs mt-1 text-surface-400">
									<span class="font-semibold">Encrypted:</span>
									{room.lastAdminInGroup ? '✅️' : '❌'}
								</div>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	</div>
{:else}
	<UserBackButton />
	<p>No rooms found</p>
{/if}

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
