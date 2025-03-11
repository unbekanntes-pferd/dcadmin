<script lang="ts">
	import {
		Accordion,
		AccordionItem,
		getToastStore,
		type AutocompleteOption
	} from '@skeletonlabs/skeleton';
	import UserSearch from './UserSearch.svelte';
	import UserIcon from '~icons/mdi/account-filter';
	import type { NodePermissionsListEntry } from '$lib/permissions/models';
	import {
		downloadAllUserPermissions,
		downloadUserPermissions,
		getPermissions
	} from '$lib/permissions';
	import PermissionsView from './UserPermissionsView.svelte';
	import CancelIcon from '~icons/mdi/cancel-circle';
	import DownloadIcon from '~icons/mdi/download';
	import Spinner from '../Spinner.svelte';
	import { save } from '@tauri-apps/plugin-dialog';
	import { ToastType, type ListParams } from '$lib/models';
	import { createToastSettings } from '$lib/utils';

	const toastStore = getToastStore();

	let downloading = false;
	let userResetFilter: () => void;
	let userFilter: number | undefined = undefined;
	let nodePermissions: NodePermissionsListEntry[] | undefined = undefined;

	let todayStr = new Date().toLocaleDateString('en-CA');

	const onUserSelection = async (e: CustomEvent<AutocompleteOption<number>>) => {
		userFilter = e.detail.value;
		await fetchUserPermissions();
	};

	const fetchUserPermissions = async () => {
		const params = {
			filter: `userId:eq:${userFilter}`
		};

		try {
			nodePermissions = await getPermissions(params);
		} catch (e) {
			const errorToast = createToastSettings(
				`Failed to fetch permissions. (${e})`,
				ToastType.Error
			);
			toastStore.trigger(errorToast);
			console.error(e);
		}
	};

	const handleDownload = async () => {
		downloading = true;
		const defaultName = `permissions-userId-${userFilter}-${todayStr}.csv`;

		const filePath = await save({
			defaultPath: defaultName,
			filters: [{ name: 'CSV', extensions: ['csv'] }]
		});

		if (filePath) {
			let params: ListParams = {
				filter: `userId:eq:${userFilter}`
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

	const handleDownloadAll = async () => {
		downloading = true;
		const defaultName = `permissions-allUsers-${todayStr}.csv`;

		const filePath = await save({
			defaultPath: defaultName,
			filters: [{ name: 'CSV', extensions: ['csv'] }]
		});

		if (filePath) {
			try {
				await downloadAllUserPermissions(filePath);
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

	const resetFilter = () => {
		userResetFilter();
		userFilter = undefined;
		nodePermissions = undefined;
	};
	
</script>

<div class="flex flex-col w-full p-4">
	<Accordion>
		<AccordionItem open>
			<svelte:fragment slot="lead"><UserIcon /></svelte:fragment>
			<svelte:fragment slot="summary">User permissions</svelte:fragment>
			<svelte:fragment slot="content">
				<UserSearch
					{onUserSelection}
					bind:resetFilter={userResetFilter}
					bind:selectedUserId={userFilter}
				/>

				<div class="flex flex-col sm:flex-row justify-between mt-4 space-y-2 sm:space-y-0">
					<!-- Wrap the Reset button in a div for alignment -->
					<div class="flex items-center sm:m-0">
						<button
							type="button"
							class="btn variant-outline-warning h-10 my-2 w-full sm:w-24 sm:my-0"
							on:click={resetFilter}
						>
							<span><CancelIcon /></span>
							<span>Reset</span>
						</button>
					</div>
					<!-- Download buttons container -->
					<div class="flex flex-col sm:flex-row space-y-2 sm:space-y-0 sm:items-center">
						<div class="flex items-center sm:m-0">
							<button
								type="button"
								class="btn variant-filled-primary h-10 my-2 w-full sm:w-fit sm:mx-2 sm:my-0"
								on:click={handleDownload}
								disabled={downloading || !nodePermissions}
							>
								{#if downloading}
									<Spinner />
								{:else}
									<span><DownloadIcon /></span>
								{/if}
								<span>Download</span>
							</button>
						</div>
						<div class="flex items-center sm:m-0">
							<button
								type="button"
								class="btn variant-filled-primary h-10 my-2 w-full sm:w-fit sm:mx-2 sm:my-0"
								on:click={handleDownloadAll}
								disabled={downloading}
							>
								{#if downloading}
									<Spinner />
								{:else}
									<span><DownloadIcon /></span>
								{/if}
								<span>Download all</span>
							</button>
						</div>
					</div>
				</div>
			</svelte:fragment>
		</AccordionItem>
	</Accordion>

	<div class="flex-1 overflow-y-auto" style="height: calc(100vh - 300px);">
		{#if userFilter && nodePermissions}
			<PermissionsView permissionsList={nodePermissions} />
		{:else}
			<div class="flex flex-row justify-center mt-4">
				<p class="text-gray-300">Select a user to view permissions</p>
			</div>
		{/if}
	</div>
</div>