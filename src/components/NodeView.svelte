<script lang="ts">
	import { goto } from '$app/navigation';
	import type { NodeInfo } from '$lib/permissions/models';
	import { toReadableSize } from '$lib/utils';
	import { Paginator, type PaginationSettings } from '@skeletonlabs/skeleton';
	import RoomIcon from '~icons/mdi/folder-account';

	export let nodes: NodeInfo[];
	export let parentId: number;

	$: paginatedNodes = nodes.slice(
		paginationSettings.page * paginationSettings.limit,
		paginationSettings.page * paginationSettings.limit + paginationSettings.limit
	);

	const handleNodeNavigation = (id: number) => {
		console.log(id);
		console.log(parentId);
		goto(`/nodes/${id}`);
	};

	let paginationSettings = {
		page: 0,
		limit: 10,
		size: nodes.length,
		amounts: [10, 25, 50]
	} satisfies PaginationSettings;

	$: paginationSettings.size = nodes.length;
	$: if (paginationSettings.page * paginationSettings.limit >= nodes.length) {
		paginationSettings.page = 0;
	}

</script>

<div class="overflow-x-auto space-y-2 h-full w-4/5 ml-4 mt-4">
	{#if parentId}
		<button on:click={() => handleNodeNavigation(parentId)}>
			<span><RoomIcon /></span>
			<span>Back</span>
		</button>
	{/if}
	<Paginator
		bind:settings={paginationSettings}
		showFirstLastButtons={false}
		showPreviousNextButtons={true}
	/>
	<div class="table-container">
		<table class="table table-hover">
			<thead>
				<tr>
					<th>Name</th>
					<th>Path</th>
					<th>Size</th>
					<th>Quota</th>
					<th>Encrypted</th>
					<th>Permissions</th>
				</tr>
			</thead>
			<tbody>
				{#each paginatedNodes as row}
					<tr>
						<td>
							{#if row.cntChildren > 0}
								<button on:click={() => handleNodeNavigation(row.id)}>
									<span><RoomIcon /></span>
									<span>
										{row.name}
									</span>
								</button>
							{:else}
								<span class="text-gray-500"><RoomIcon /></span>
								<span class="text-gray-500">
									{row.name}
								</span>
							{/if}
						</td>
						<td>{row.parentPath}</td>
						<td>{row.size ? toReadableSize(row.size) : 0}</td>
						<td>{row.quota ? toReadableSize(row.quota) : 'N/A'}</td>
						<td>{row.isEncrypted ? '✅️' : '❌'}</td>
						<td>
							{row.cntPermissions}
						</td>
					</tr>
				{/each}
			</tbody>
		</table>
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
