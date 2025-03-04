<script lang="ts">
	import { page } from '$app/state';
	import { ToastType, type ListParams } from '$lib/models';
	import { downloadUserPermissions, getPermissions } from '$lib/permissions';
	import { createToastSettings } from '$lib/utils';
	import { getToastStore } from '@skeletonlabs/skeleton';
	import type { NodePermissionsListEntry, UserPermissions } from '$lib/permissions/models';
	import NodePermissionsView from '../../../../components/nodes/NodePermissionsView.svelte';
	import NodeInfoView from '../../../../components/nodes/NodeInfoView.svelte';
	import { save } from '@tauri-apps/plugin-dialog';
	import DownloadIcon from '~icons/mdi/download';
	import Spinner from '../../../../components/Spinner.svelte';
	import NodeBackButton from '../../../../components/nodes/NodeBackButton.svelte';
	import type { LastAdminUserRoomList } from '$lib/users/models';
	import { getLastAdminUserRooms } from '$lib/users';
	import UserBackButton from '../../../../components/users/UserBackButton.svelte';
	import UserLastAdminRoomsView from '../../../../components/users/UserLastAdminRoomsView.svelte';

	const toastStore = getToastStore();
	let downloading = false;

    let lastAdminRooms: LastAdminUserRoomList | undefined = undefined;
	const todayStr = new Date().toLocaleDateString('en-CA');

	const fetchLastAdminRooms = async (
		userId: number
	): Promise<LastAdminUserRoomList> => {

		const lastAdminRooms = getLastAdminUserRooms(userId);

		return lastAdminRooms;
	};

	$: {
		const userId = parseInt(page.params.id, 10);
		fetchLastAdminRooms(userId)
			.then((result) => {
				lastAdminRooms = result;
			})
			.catch((err) => {
				console.error(err);
				let errorToast = createToastSettings('Error fetching user last admin rooms', ToastType.Error);
				toastStore.trigger(errorToast);
			});
	}

</script>


	<div class="flex flex-col w-full p-4">
        <div class="card p-4 overflow-y-visible w-full mb-4">
            <div class="flex flex-col sm:flex-row justify-between items-center mt-4 space-y-2 sm:space-y-0">
                <div>
                    <UserBackButton full />
                </div>
            </div>
        </div>
        {#if lastAdminRooms && lastAdminRooms.items.length > 0}
		<div class="flex-1 overflow-y-auto" style="height: calc(100vh - 400px);">
			<UserLastAdminRoomsView roomList={lastAdminRooms} />
		</div>
        {/if}
    </div>
{#if !lastAdminRooms || lastAdminRooms.items.length === 0}
	<div class="flex justify-center items-center w-full p-4">
		<p>No permissions found</p>
	</div>
{/if}