<script lang="ts">
	import { save } from '@tauri-apps/api/dialog';
	import { downloadEvents, getEvents, getOperationTypes } from '$lib/events';
	import type { EventList, EventParams, OperationTypeList } from '$lib/events/models';
	import { onMount } from 'svelte';
	import Events from '../../components/Events.svelte';
	import {
		Paginator,
		type PaginationSettings,
		type AutocompleteOption,
		Accordion,
		AccordionItem,
		getToastStore
	} from '@skeletonlabs/skeleton';
	import CancelIcon from '~icons/mdi/cancel-circle';
	import DownloadIcon from '~icons/mdi/download';
	import CalenderFilterIcon from '~icons/mdi/calendar-filter';
	import Spinner from '../../components/Spinner.svelte';
	import UserSearch from '../../components/UserSearch.svelte';
	import { createToastSettings } from '$lib/utils';
	import { ToastType } from '$lib/models';

	const toastStore = getToastStore();

	let eventList: EventList | null;
	let operations: OperationTypeList | null;
	let operationFilter: number | null = null;
	let userFilter: number | undefined = undefined;
	let userResetFilter: () => void;

	let toDate = new Date();
	let fromDate = new Date(toDate);
	fromDate.setDate(fromDate.getDate() - 7);

	$: ({ page, limit } = paginationSettings);

	$: toDateFilter = new Date(toDateStr);
	$: fromDateFilter = new Date(fromDateStr);
	let fromDateStr = fromDate.toLocaleDateString('en-CA');
	let toDateStr = toDate.toLocaleDateString('en-CA');

	let paginationSettings = {
		page: 0,
		limit: 10,
		size: 0,
		amounts: [10, 20, 50]
	} satisfies PaginationSettings;

	let loading = true;
	let downloading = false;

	const fetchEvents = async () => {
		loading = true;
		try {
			let params: EventParams = {
				offset: page * limit,
				limit: limit,
				fromDate: fromDateFilter.toISOString(),
				toDate: toDateFilter.toISOString()
			};

			if (operationFilter) {
				params.operationType = operationFilter;
			}

			if (userFilter) {
				params.userId = userFilter;
			}

			eventList = await getEvents({ ...params });
			paginationSettings.size = eventList.range.total;
		} catch (e) {
			const errorToast = createToastSettings(`Failed to fetch events. (${e})`, ToastType.Error);
			toastStore.trigger(errorToast);
		} finally {
			loading = false;
		}
	};

	const onUserSelection = async (e: CustomEvent<AutocompleteOption<number>>) => {
		userFilter = e.detail.value;
		await fetchEvents();
	};

	const onPageChange = async (e: CustomEvent) => {
		paginationSettings.page = e.detail;
		await fetchEvents();
	};

	const onAmountChange = async (e: CustomEvent) => {
		paginationSettings.limit = e.detail;
		paginationSettings.page = 0;
		await fetchEvents();
	};

	const resetFilters = async (e: MouseEvent) => {
		if (userResetFilter) userResetFilter();
		operationFilter = null;
		userFilter = undefined;
		toDate = new Date();
		fromDate = new Date(toDate);
		fromDate.setDate(fromDate.getDate() - 7);
		await fetchEvents();
	};

	const handleDownload = async () => {
		downloading = true;
		const defaultName = `events-${fromDateFilter.toLocaleDateString('en-CA')}-${toDateFilter.toLocaleDateString('en-CA')}.csv`;

		const filePath = await save({
			defaultPath: defaultName,
			filters: [{ name: 'CSV', extensions: ['csv'] }]
		});

		if (filePath) {
			let params: EventParams = {
				offset: 0,
				limit: 500,
				fromDate: fromDateFilter.toISOString(),
				toDate: toDateFilter.toISOString()
			};

			if (operationFilter) {
				params.operationType = operationFilter;
			}

			if (userFilter) {
				params.userId = userFilter;
			}

			try {
				await downloadEvents(filePath, params);
				const successToast = createToastSettings(
					`Download of ${filePath} complete.`,
					ToastType.Success
				);
				toastStore.trigger(successToast);
			} catch (err) {
				const errorToast = createToastSettings(`Download failed. (${err})`, ToastType.Error);
				toastStore.trigger(errorToast);
			} finally {
				downloading = false;
			}
		}

		downloading = false;
	};

	onMount(async () => {
		operations = await getOperationTypes();
		await fetchEvents();
	});
</script>

<div class="flex flex-col w-full p-4">
	<Accordion>
		<AccordionItem>
			<svelte:fragment slot="lead"><CalenderFilterIcon /></svelte:fragment>
			<svelte:fragment slot="summary">Event options</svelte:fragment>
			<svelte:fragment slot="content">
				<div class="card p-4 overflow-y-visible w-full">
					<header class="card-header flex flex-col w-full items-center mb-2 pt-0">
						<div class="mb-2">
							<b>Search for events</b>
						</div>
					</header>
					<div>
						<div class="flex flex-row w-full">
							<div class="flex flex-col w-1/2">
								<div class="flex flex-col w-fit">
									{#if operations && operations.operations}
										<label for="operation-filter">Actions</label>
										<select
											id="operation-filter"
											class="select w-full"
											size="3"
											bind:value={operationFilter}
											on:change={fetchEvents}
										>
											{#each operations.operations as operation}
												<option value={operation.id}>{operation.name}</option>
											{/each}
										</select>
									{/if}
									<div class="flex flex-row my-2 w-full">
										<div class="w-1/2">
											<label for="date-from">From</label>

											<input
												id="date-from"
												type="date"
												class="input"
												bind:value={fromDateStr}
												on:change={fetchEvents}
											/>
										</div>

										<div class="w-1/2 ml-4">
											<label for="date-to" class="">To</label>
											<input
												id="date-to"
												type="date"
												class="input"
												bind:value={toDateStr}
												on:change={fetchEvents}
											/>
										</div>
									</div>
								</div>
							</div>

							<UserSearch
								{onUserSelection}
								bind:resetFilter={userResetFilter}
								bind:selectedUserId={userFilter}
							/>
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
			</svelte:fragment>
		</AccordionItem>
	</Accordion>
	{#if loading}
		<div class="flex justify-center items-center">
			<div class="spinner"></div>
		</div>
	{:else if eventList}
		<div class="my-4">
			<Paginator
				bind:settings={paginationSettings}
				on:page={onPageChange}
				on:amount={onAmountChange}
				controlVariant="variant-outline"
			></Paginator>
		</div>
		<Events {eventList}></Events>
	{/if}
</div>
