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


<div class="overflow-x-auto space-y-2 h-full">
	<div class="table-container">
		<table class="table table-hover">
			<thead>
				<tr>
					<th>Time</th>
					<th>Message</th>
					<th>User</th>
					<th>Action</th>
					<th>Status</th>
				</tr>
			</thead>
			<tbody>
				{#each eventList.events as row}
					<tr>
						<td>{formatUTCDateTime(row.time)}</td>
						<td>{row.message}</td>
						<td>{row.userName ? row.userName : 'n/a'}</td>
						<td>{row.operationName}</td>
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
