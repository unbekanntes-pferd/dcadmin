<script lang="ts">
	import type { EventList } from '$lib/events/models';
	import { formatUTCDateTime } from '$lib/utils';

	export let eventList: EventList;

	const getStatusClass = (status?: string) => {
		if (status === 'success') {
			return 'text-green-300';
		} else if (status === 'error') {
			return 'text-red-300';
		} else {
			return 'text-surface-300';
		}
	};

</script>


<div class="space-y-2">
	<div class="table-container">
		<table class="table table-hover min-w-full">
			<thead>
				<tr>
					<th class="hidden md:table-cell">Time</th>
					<th>Message</th>
					<th class="hidden sm:table-cell">User</th>
					<th class="hidden lg:table-cell">Action</th>
					<th>Status</th>
				</tr>
			</thead>
			<tbody>
				{#each eventList.events as row}
					<tr>
						<td class="hidden md:table-cell">{formatUTCDateTime(row.time)}</td>
						<td>
							<div class="md:hidden text-xs text-surface-400 mb-1">{formatUTCDateTime(row.time)}</div>
							{row.message}
							<div class="sm:hidden text-xs mt-1">
								<span class="font-semibold">User:</span> {row.userName ? row.userName : 'n/a'}
							</div>
							<div class="lg:hidden text-xs mt-1">
								<span class="font-semibold">Action:</span> {row.operationName}
							</div>
						</td>
						<td class="hidden sm:table-cell">{row.userName ? row.userName : 'n/a'}</td>
						<td class="hidden lg:table-cell">{row.operationName}</td>
						<td class={getStatusClass(row.status)}>{row.status ? row.status : 'n/a'}</td>
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
