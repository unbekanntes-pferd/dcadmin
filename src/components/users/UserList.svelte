<script lang="ts">
	import { roleList, type UserList } from '$lib/users/models';
	import {
		Accordion,
		AccordionItem,
		getToastStore,
		Paginator,
		SlideToggle,
		type PaginationSettings
	} from '@skeletonlabs/skeleton';
	import Spinner from '../Spinner.svelte';
	import UserFilterIcon from '~icons/mdi/user-search';
	import CancelIcon from '~icons/mdi/cancel-circle';
	import DownloadIcon from '~icons/mdi/download';
	import { ToastType, type ListParams } from '$lib/models';
	import { createToastSettings } from '$lib/utils';
	import UserListView from './UserListView.svelte';
	import { onMount } from 'svelte';
	import { downloadUsers, getUsers } from '$lib/users';
	import { save } from '@tauri-apps/api/dialog';

	let userList: UserList | null;
	let downloading = false;
	let loading = false;
	let roleFilters: string[] = [];
	let lockedFilter = false;
	let accordionOpen = true;
	const toastStore = getToastStore();

	let paginationSettings = {
		page: 0,
		limit: 10,
		size: 0,
		amounts: [10, 20, 50]
	} satisfies PaginationSettings;

	$: ({ page, limit } = paginationSettings);

	let todayStr = new Date().toLocaleDateString('en-CA');

	const onPageChange = async (e: CustomEvent) => {
		paginationSettings.page = e.detail;
		await fetchUsers();
	};

	const onAmountChange = async (e: CustomEvent) => {
		paginationSettings.limit = e.detail;
		paginationSettings.page = 0;
		await fetchUsers();
	};

	const fetchUsers = async () => {
		loading = true;

		try {
			let params: ListParams = {
				offset: page * limit,
				limit: limit
			};

			if (roleFilters && roleFilters.length > 0) {
				let filter = roleFilters.map((role) => `hasRole:eq:${role}`);
				params.filter = filter.join('|');
			}

			if (lockedFilter) {
				params.filter = params.filter ? `${params.filter}|isLocked:eq:true` : 'isLocked:eq:true';
			}

			userList = await getUsers(params);
			paginationSettings.size = userList.range.total;
		} catch (e) {
			const errorToast = createToastSettings(`Failed to fetch events. (${e})`, ToastType.Error);
			toastStore.trigger(errorToast);
		} finally {
			loading = false;
		}
	};

	const resetFilters = async () => {
		roleFilters = [];
		await fetchUsers();
	};

	const handleDownload = async () => {
		downloading = true;
		const defaultName = `users-${todayStr}.csv`;

		const filePath = await save({
			defaultPath: defaultName,
			filters: [{ name: 'CSV', extensions: ['csv'] }]
		});

		if (filePath) {
			let params: ListParams = {
				offset: 0,
				limit: 500
			};

			if (roleFilters && roleFilters.length > 0) {
				let filter = roleFilters.map((role) => `hasRole:eq:${role}`);
				params.filter = filter.join('|');
			}

			try {
				await downloadUsers(params, filePath);
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

	onMount(async () => {
		await fetchUsers();
	});
</script>

<div class="flex flex-col w-full p-4">
	<Accordion class="mb-4">
		<AccordionItem bind:open={accordionOpen}>
			<svelte:fragment slot="lead"><UserFilterIcon /></svelte:fragment>
			<svelte:fragment slot="summary">User options</svelte:fragment>
			<svelte:fragment slot="content">
				<div class="card p-4 overflow-y-visible w-full">
					<header class="card-header flex flex-col w-full items-center mb-2 pt-0">
						<div class="mb-2">
							<b>Search for users</b>
						</div>
					</header>
					<div>
						<div class="flex flex-col w-full">
							<div class="flex flex-col w-1/2">
								{#if roleList}
									<label for="operation-filter">Roles</label>
									<select
										id="operation-filter"
										class="select w-full"
										size="3"
										bind:value={roleFilters}
										on:change={fetchUsers}
										multiple
									>
										{#each roleList as role}
											<option value={role.value}>{role.label}</option>
										{/each}
									</select>
								{/if}
							</div>
							<div class="mt-4">
								<SlideToggle
									name="slider-label"
									bind:checked={lockedFilter}
									active="bg-primary-600"
									on:change={fetchUsers}>🔒Locked</SlideToggle
								>
							</div>
							<div class="flex flex-row justify-between mt-4">
								<button
									type="button"
									class="btn variant-outline-warning my-2 w-24"
									on:click={resetFilters}
								>
									<span><CancelIcon /></span>
									<span>Reset</span>
								</button>
								<button
									type="button"
									class="btn variant-filled-primary my-2 w-fit mx-2"
									on:click={handleDownload}
									disabled={downloading}
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
				</div></svelte:fragment
			>
		</AccordionItem>
	</Accordion>

	<div>
		<div class="my-4">
			<Paginator
				bind:settings={paginationSettings}
				on:page={onPageChange}
				on:amount={onAmountChange}
				controlVariant="variant-outline"
			></Paginator>
		</div>
	</div>
	<div class="flex-1 overflow-y-auto" style="{accordionOpen ? 'height: 440px' : 'height: 760px'}">
		{#if loading}
			<div class="flex justify-center items-center">
				<Spinner />
			</div>
		{:else if userList}
			<UserListView {userList} />
		{/if}
	</div>
</div>
