<script lang="ts">
	import { userAccount } from '../stores/auth';
	import { getCustomerInfo } from '$lib/customer';
	import SettingsIcon from '~icons/mdi/settings';
	import RoomsConfigIcon from '~icons/mdi/folder-cog';
	import UserIcon from '~icons/mdi/user';
	import GroupIcon from '~icons/mdi/account-group';
	import AuditorIcon from '~icons/mdi/event-search';
	import { popup, type PopupSettings } from '@skeletonlabs/skeleton';
	import { toReadableSize } from '$lib/utils';
	import UsersIcon from '~icons/mdi/users';
	import SpaceIcon from '~icons/mdi/database';
	import { getVersion } from '@tauri-apps/api/app';

	const firstName = $userAccount!.firstName;
	const isConfigManager = $userAccount!.isConfigManager;
	const isRoomManager = $userAccount!.isRoomManager;
	const isUserManager = $userAccount!.isUserManager;
	const isGroupManager = $userAccount!.isGroupManager;
	const isAuditor = $userAccount!.isAuditor;
	const userRoles = [
		$userAccount!.isConfigManager,
		$userAccount!.isRoomManager,
		$userAccount!.isUserManager,
		$userAccount!.isGroupManager,
		$userAccount!.isAuditor
	];

	const popUps = ['Config Manager', 'Room Manager', 'User Manager', 'Group Manager', 'Auditor'];

	const popupSettings: PopupSettings[] = popUps.map((popup) => {
		return {
			event: 'hover',
			target: popup,
			placement: 'top'
		};
	});
</script>

{#each popupSettings as settings, idx}
	{#if userRoles[idx]}
		<div data-popup={settings.target} class="p-2 bg-surface-300 rounded-md">
			<span>{settings.target}</span>
		</div>
	{/if}
{/each}

<div class="card p-4 w-full m-4">
	<div class="flex flex-row justify-between items-center">
		<div>
			<header class="card-header">
				<div class="my-2">
					<b>Welcome, {firstName}.</b>
				</div>
				<div class="flex flex-row">
					{#if isConfigManager}
						<div use:popup={popupSettings[0]}>
							<SettingsIcon class="text-green-300 mx-1" />
						</div>
					{/if}
					{#if isRoomManager}
						<div use:popup={popupSettings[1]}>
							<RoomsConfigIcon class="text-green-300 mx-1" />
						</div>
					{/if}
					{#if isUserManager}
						<div use:popup={popupSettings[2]}>
							<UserIcon class="text-green-300 mx-1" />
						</div>
					{/if}
					{#if isGroupManager}
						<div use:popup={popupSettings[3]}><GroupIcon class="text-green-300 mx-1" /></div>
					{/if}
					{#if isAuditor}
						<div use:popup={popupSettings[4]}>
							<AuditorIcon class="text-green-300 mx-1" />
						</div>
					{/if}
				</div>
			</header>

			<div class="flex flex-row justify-between">
				<div class="p-4">
					{#await getCustomerInfo()}
						<p>Loading...</p>
					{:then customerInfo}
						<div class="flex flex-row">
							<UsersIcon class="mx-2" />
							<p>{customerInfo.userCount} | {customerInfo.userLimit}</p>
						</div>

						<div class="flex flex-row">
							<SpaceIcon class="mx-2" />
							<p>
								{toReadableSize(customerInfo.spaceUsed)} | {toReadableSize(customerInfo.spaceLimit)}
							</p>
						</div>
					{:catch error}
						<p style="color: red">{error.message}</p>
					{/await}
				</div>
			</div>
		</div>
		<div>
			<img src="/dcadmin_logo.png" alt="dcadmin logo" width="200" height="200" />
		</div>
	</div>
<footer class="card-footer text-xs text-gray-500">
	{#await getVersion() then version}
	dcadmin {version}
	{:catch}
		<p class="text-xs text-red-400">version not available</p>
	{/await}

</footer>
</div>
