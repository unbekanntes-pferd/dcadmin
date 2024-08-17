<script lang="ts">
	import type { NodePermissionsListEntry } from '$lib/permissions/models';
	import { toReadableSize } from '$lib/utils';
	import RoomIcon from '~icons/mdi/folder-account';

	export let nodeInfo: NodePermissionsListEntry;
</script>

<header class="card-header flex flex-col w-full items-center mb-2 pt-0">
	<div class="mb-2 flex flex-row">
		<span class="mr-2">
			<RoomIcon />
		</span>
		<b> {nodeInfo.nodeName}</b>
	</div>
	<div>
		{nodeInfo.nodeParentPath}
	</div>
</header>
<div>
	<div>
		{toReadableSize(nodeInfo.nodeSize ? nodeInfo.nodeSize : 0)}
	</div>
	<div>
		<div>
			{nodeInfo.nodeQuota ? toReadableSize(nodeInfo.nodeQuota) : 'Unlimited'}
		</div>
	</div>
</div>
