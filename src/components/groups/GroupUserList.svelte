<script lang="ts">
	import { page as appPage } from '$app/stores';
	import { downloadGroupUsers, getGroup, getGroupUsers } from '$lib/groups';
	import type { GroupInfo, GroupUser } from '$lib/groups/models';
	import { ToastType, type ListParams } from '$lib/models';
	import { createToastSettings } from '$lib/utils';
	import { getToastStore, Paginator, type PaginationSettings } from '@skeletonlabs/skeleton';
	import Spinner from '../Spinner.svelte';
	import GroupIcon from '~icons/mdi/account-group';
	import GroupBackButton from './GroupBackButton.svelte';
	import DownloadIcon from '~icons/mdi/download';
	import { save } from '@tauri-apps/plugin-dialog';
	import { onMount } from 'svelte';

	let loading = false;
	const toastStore = getToastStore();
	let groupInfo: GroupInfo | undefined = undefined;
	let groupUsers: GroupUser[] = [];
	let downloading = false;
	let todayStr = new Date().toLocaleDateString('en-CA');

	let paginationSettings = {
		page: 0,
		limit: 10,
		size: 0,
		amounts: [10, 20, 50]
	} satisfies PaginationSettings;

	let totalUsers = 0;

	$: groupId = parseInt($appPage.params.id, 10);

	$: ({ page, limit } = paginationSettings);

	const onPageChange = async (e: CustomEvent) => {
		paginationSettings.page = e.detail;
		await fetchGroupUsers();
	};

	const onAmountChange = async (e: CustomEvent) => {
		paginationSettings.limit = e.detail;
		paginationSettings.page = 0;
		await fetchGroupUsers();
	};

    const fetchGroupInfo = async () => {
        groupInfo = await getGroup(groupId);
    };

    const fetchGroupUsers = async () => {
        loading = true;
        const params = { offset: page * limit, limit: limit };
        console.log(params);
        const userList = await getGroupUsers(groupId, params);
        console.log(userList);
        groupUsers = userList.items;
        paginationSettings.size = userList.range.total;
        totalUsers = userList.range.total;
        loading = false;
    };

	onMount(async () => {
        loading = true;
		await fetchGroupInfo();
        await fetchGroupUsers();
        loading = false;
	});

	const handleDownload = async () => {
		downloading = true;
		const defaultName = `group-users-group-${groupId}-${todayStr}.csv`;

		const filePath = await save({
			defaultPath: defaultName,
			filters: [{ name: 'CSV', extensions: ['csv'] }]
		});

		if (filePath) {
			let params: ListParams = {
				offset: 0,
				limit: 500
			};

			try {
				await downloadGroupUsers(groupId, params, filePath);
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

{#if !loading && groupInfo && groupUsers.length > 0}
	<div class="flex flex-col mx-4 mt-4 w-full">
		<div class="card p-4 overflow-y-visible w-full mb-4">
			<header class="card-header flex flex-col w-full items-center mb-2 pt-0">
				<div class="mb-2 flex flex-row">
					<span class="mr-2">
						<GroupIcon />
					</span>
					<b> {groupInfo.name}</b>
				</div>
				<div>
					{totalUsers} users
				</div>
			</header>
			<div class="flex flex-row justify-between items-center mt-4">
				<div>
					<GroupBackButton full />
				</div>
				<div class="flex flex-row justify-end items-center">
					<button
						type="button"
						class="btn variant-filled-primary my-2 w-fit mx-2"
						on:click={handleDownload}
						disabled={downloading || groupUsers.length === 0}
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

		<div>
			<div class="my-4">
				<Paginator
					bind:settings={paginationSettings}
					on:page={onPageChange}
					on:amount={onAmountChange}
					controlVariant="variant-outline"
				></Paginator>
			</div>

			<div class="flex-1 overflow-y-auto" style="height: 770px;">
				<div class="table-container h-full overflow-y-auto">
					<table class="table table-hover">
						<thead>
							<tr>
								<th>Name</th>
								<th>Username</th>
								<th>Email</th>
							</tr>
						</thead>
						<tbody>
							{#each groupUsers as row}
								<tr>
									<td>
										<span>
											{row.firstName ? row.firstName : ''}
											{row.lastName ? row.lastName : ''}
										</span>
									</td>
									<td>
										<span>
											{row.userName ? row.userName : ''}
										</span>
									</td>
									<td>
										<span>
											{row.email ? row.email : ''}
										</span>
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			</div>
		</div>
	</div>
{:else}
	<div class="flex justify-center items-center h-full w-full">
		<Spinner width="w-36" />
	</div>
{/if}
