<script lang="ts">
	import { page } from '$app/stores';
	import { ToastType, type ListParams } from '$lib/models';
	import { downloadUserPermissions, getPermissions } from '$lib/permissions';
	import { createToastSettings } from '$lib/utils';
	import { getToastStore } from '@skeletonlabs/skeleton';
	import type { NodePermissionsListEntry, UserPermissions } from '$lib/permissions/models';
	import NodePermissionsView from '../../../../components/NodePermissionsView.svelte';
	import NodeInfoView from '../../../../components/NodeInfoView.svelte';
	import { save } from '@tauri-apps/api/dialog';
	import DownloadIcon from '~icons/mdi/download';
	import Spinner from '../../../../components/Spinner.svelte';
	import NodeBackButton from '../../../../components/NodeBackButton.svelte';

	const toastStore = getToastStore();
	let downloading = false;

	let userPermissions: UserPermissions[] = [];
	let nodeInfo: NodePermissionsListEntry | undefined = undefined;
	const todayStr = new Date().toLocaleDateString('en-CA');

	const fetchNodeAndPermissions = async (
		nodeId: number
	): Promise<{ permissions: UserPermissions[]; node: NodePermissionsListEntry }> => {
		const params = {
			filter: `nodeId:eq:${nodeId}`
		};

		const permissions = await getPermissions(params);

		return {
			permissions: permissions[0].userPermissions,
			node: permissions[0]
		};
	};

	$: {
		const nodeId = parseInt($page.params.id, 10);
		fetchNodeAndPermissions(nodeId)
			.then((result) => {
				userPermissions = result.permissions;
				nodeInfo = result.node;
			})
			.catch((err) => {
				console.error(err);
				let errorToast = createToastSettings('Error fetching user permissions', ToastType.Error);
				toastStore.trigger(errorToast);
			});
	}

	const handleDownload = async () => {
		downloading = true;
		const defaultName = `permissions-nodeId-${$page.params.id}-${todayStr}.csv`;

		const filePath = await save({
			defaultPath: defaultName,
			filters: [{ name: 'CSV', extensions: ['csv'] }]
		});

		if (filePath) {
			let params: ListParams = {
				filter: `nodeId:eq:${$page.params.id}`
			};

			try {
				await downloadUserPermissions(filePath, params);
				let successToast = createToastSettings(
					`Download of ${filePath} complete.`,
					ToastType.Success
				);
				toastStore.trigger(successToast);
			} catch (err) {
				let errorToast = createToastSettings(`Download failed. (${err})`, ToastType.Error);
				toastStore.trigger(errorToast);
			} finally {
				downloading = false;
			}
		}

		downloading = false;
	};
</script>

{#if userPermissions.length > 0 && nodeInfo}
	<div class="flex flex-col ml-4 mt-4 w-4/5">
        <div class="card p-4 overflow-y-visible w-full mb-4">
            <NodeInfoView {nodeInfo} />
            <div class="flex flex-row justify-between items-center mt-4">
                <div>
                    <NodeBackButton full />
                </div>
                <div class="flex flex-row justify-end items-center">
                    <button
                        type="button"
                        class="btn variant-filled-primary my-2 w-fit mx-2"
                        on:click={handleDownload}
                        disabled={downloading || !userPermissions}
                    >
                        {#if downloading}
                            <Spinner />
                        {:else}
                            <span><DownloadIcon /></span>
                        {/if}
                        <span>Download</span>
                    </button>
                </div>
            </div>
        </div>
		<NodePermissionsView {userPermissions} />
	</div>
{:else}
	<p>No permissions found</p>
{/if}
