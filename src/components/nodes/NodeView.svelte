<script lang="ts">
	import { goto } from '$app/navigation';
	import type { NodeInfo } from '$lib/permissions/models';
	import { handleNodeNavigation, toReadableSize } from '$lib/utils';
	import { Paginator, type PaginationSettings } from '@skeletonlabs/skeleton';
	import RoomIcon from '~icons/mdi/folder-account';
	import UsersIcon from '~icons/mdi/users';
	import { page } from '$app/state';
	import NodeBackButton from './NodeBackButton.svelte';
	import { lastNodesPage } from '../../stores/nodes';

	export let nodes: NodeInfo[];

	$: paginatedNodes = nodes.slice(
		paginationSettings.page * paginationSettings.limit,
		paginationSettings.page * paginationSettings.limit + paginationSettings.limit
	);

	let paginationSettings = {
		page: $lastNodesPage,
		limit: 10,
		size: nodes.length,
		amounts: [10, 25, 50]
	} satisfies PaginationSettings;

	$: paginationSettings.size = nodes.length;
	$: $lastNodesPage = paginationSettings.page;
	$: if (paginationSettings.page * paginationSettings.limit >= nodes.length) {
		paginationSettings.page = 0;
	}

</script>

<div class="flex flex-col w-full p-4 space-y-2">
	{#if page.url.pathname !== '/nodes/0'}
		<NodeBackButton />
	{/if}

	<div class="my-4">
		<Paginator
			bind:settings={paginationSettings}
			showFirstLastButtons={false}
			showPreviousNextButtons={true}
		/>
	</div>
	
	<div class="flex-1 overflow-y-auto" style="height: calc(100vh - 300px);">
		<div class="table-container">
			<table class="table table-hover min-w-full">
				<thead>
					<tr>
						<th>Name</th>
						<th class="hidden md:table-cell">Path</th>
						<th class="hidden sm:table-cell">Size</th>
						<th class="hidden lg:table-cell">Quota</th>
						<th class="hidden sm:table-cell">Encrypted</th>
						<th>Permissions</th>
					</tr>
				</thead>
				<tbody>
					{#each paginatedNodes as row}
						<tr>
							<td class="w-max">
								{#if row.cntChildren > 0}
									<button on:click={() => handleNodeNavigation(row.id)}>
										<div class="flex flex-row">
											<span class="mr-2"><RoomIcon /></span>
											<span>
												{row.name}
											</span>
										</div>
									</button>
								{:else}
									<div class="flex flex-row">
										<span class="text-gray-500 mr-2"><RoomIcon /></span>
										<span class="text-gray-500">
											{row.name}
										</span>
									</div>
								{/if}
								<div class="md:hidden text-xs mt-1 text-surface-400">
									<span class="font-semibold">Path:</span> {row.parentPath}
								</div>
								<div class="sm:hidden text-xs mt-1 text-surface-400">
									<span class="font-semibold">Size:</span> {row.size ? toReadableSize(row.size) : 0}
								</div>
								<div class="lg:hidden text-xs mt-1 text-surface-400">
									<span class="font-semibold">Quota:</span> {row.quota ? toReadableSize(row.quota) : 'N/A'}
								</div>
								<div class="sm:hidden text-xs mt-1 text-surface-400">
									<span class="font-semibold">Encrypted:</span> {row.isEncrypted ? '✅️' : '❌'}
								</div>
							</td>
							<td class="hidden md:table-cell">{row.parentPath}</td>
							<td class="hidden sm:table-cell">{row.size ? toReadableSize(row.size) : 0}</td>
							<td class="hidden lg:table-cell">{row.quota ? toReadableSize(row.quota) : 'N/A'}</td>
							<td class="hidden sm:table-cell">{row.isEncrypted ? '✅️' : '❌'}</td>
							<td>
								<button on:click={() => handleNodeNavigation(row.id, true)}>
									<span><UsersIcon /></span>
									<span>
										{row.cntPermissions}
									</span>
								</button>
							</td>
						</tr>
					{/each}
				</tbody>
			</table>
		</div>
	</div>
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
