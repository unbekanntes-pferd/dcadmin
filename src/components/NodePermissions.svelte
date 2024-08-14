<script lang="ts">
	import { getPermissions } from "$lib/permissions";
	import { intoNodeInfo } from "$lib/permissions/models";
	import NodeView from "./NodeView.svelte";


    let nodeId = 0;
    let nodes = undefined;

    const fetchNodeInfo = async () => {
		const params = {
			filter: `nodeParentId:eq:${nodeId}`
		};

		const permissions = await getPermissions(params);
		return permissions.map(intoNodeInfo);
	};

</script>

{#if nodes}
<NodeView {nodes}/>
{/if}