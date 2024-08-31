<script lang="ts">
	import { page } from '$app/stores';
	import { getPermissions } from '$lib/permissions';
	import { intoNodeInfo, type NodeInfo } from '$lib/permissions/models';
	import { getToastStore } from '@skeletonlabs/skeleton';
	import NodeView from '../../../components/nodes/NodeView.svelte';
	import Spinner from '../../../components/Spinner.svelte';
	import { createToastSettings } from '$lib/utils';
	import { ToastType } from '$lib/models';

	let nodes: NodeInfo[] = [];
	const toastStore = getToastStore();

	$: {
		const parentId = parseInt($page.params.id, 10);
		fetchNodeInfo(parentId)
			.then((result) => {
				nodes = result;
			})
			.catch(() => {
				let errorToast = createToastSettings('Error fetching nodes', ToastType.Error);
				toastStore.trigger(errorToast);
			});
	}

	const fetchNodeInfo = async (parentId: number) => {
		const params = {
			filter: `nodeParentId:eq:${parentId}`
		};

		try {
			const permissions = await getPermissions(params);
			return permissions.map(intoNodeInfo);
		} catch (error) {
			let errorToast = createToastSettings('Error fetching nodes', ToastType.Error);
			toastStore.trigger(errorToast);
		}

		return [];
	};
</script>

{#if nodes.length > 0}
	<NodeView {nodes} />
{:else}
	<div class="flex justify-center w-full h-full">
		<Spinner width='w-36'  />
	</div>
{/if}
