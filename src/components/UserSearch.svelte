<script lang="ts">
	import { ToastType } from '$lib/models';
	import { getUsers } from '$lib/users';
	import type { UserList } from '$lib/users/models';
	import { createToastSettings } from '$lib/utils';
	import { Autocomplete, getToastStore, type AutocompleteOption } from '@skeletonlabs/skeleton';

	const toastStore = getToastStore();

	export let onUserSelection: (e: CustomEvent<AutocompleteOption<number>>) => void;

	export const resetFilter = () => {
		userSearch = '';
		selected = false;
		selectedUserId = undefined;
	};

	export let selectedUserId: number | undefined;

	let userList: UserList | null;
	let userSearch: string = '';
	$: autocompleteUserList = userList ? buildUserAutoCompleteOptions(userList) : [];
	$: userFilterQuery = userSearch
		? `userName:cn:${userSearch}|firstName:cn:${userSearch}|lastName:cn:${userSearch}`
		: undefined;
	$: userListParams = {
		filter: userFilterQuery,
		offset: 0,
		limit: 10
	};

	let selected = false;

	const fetchUsers = async () => {
		try {
			userList = await getUsers(userListParams);
		} catch (e) {
			const errorToast = createToastSettings(`Failed to fetch users. (${e})`, ToastType.Error);
			toastStore.trigger(errorToast);
			console.error(e);
		}
	};

	const buildUserAutoCompleteOptions = (userList: UserList): AutocompleteOption<number>[] => {
		let newList = userList.items.map((user) => {
			return {
				value: user.id,
				label: `${user.userName}`
			};
		});

		return newList;
	};

	const internalOnUserSelection = (e: CustomEvent<AutocompleteOption<number>>) => {
		onUserSelection(e);
		userSearch = e.detail.label;
		selectedUserId = e.detail.value;
		selected = true;
	};
</script>

<div class="flex flex-col w-1/2 overflow-y-visible">
	<label for="user-filter">User search</label>

	<div class="relative">
		<input
			id="user-filter"
			class="input h-12 mb-4"
			type="text"
			name="user-filter"
			bind:value={userSearch}
			on:input={fetchUsers}
			placeholder="Username, first name or last name"
		/>

		{#if userSearch && !selected}
			<div class="card max-h-48 p-4! overflow-y-auto absolute top left" tabindex="-1">
				<Autocomplete
					bind:input={userSearch}
					options={autocompleteUserList}
					on:selection={internalOnUserSelection}
				/>
			</div>
		{/if}
	</div>
</div>
