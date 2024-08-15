<script lang="ts">
	import { displayPermissionsTemplate } from '$lib/permissions';
	import { type NodePermissions, type UserPermissions } from '$lib/permissions/models';
	import { popup, type PopupSettings } from '@skeletonlabs/skeleton';

	export let userPermissions: UserPermissions[];

	$: popupSettings = userPermissions.map((permissions) => {
		return {
			event: 'hover',
			target: getPopupTarget(permissions),
			placement: 'bottom'
		} as PopupSettings;
	});

	const getPopupTarget: (row: UserPermissions) => string = (row) => {
		return `${row.userId}`;
	};

	const getPermissionsByTarget: (target: string) => NodePermissions | undefined = (target) => {
		const row = userPermissions.find((row) => getPopupTarget(row) === target);
		return row?.permissions;
	};

	const findSettingsByTarget: (target: string) => PopupSettings = (target) => {
		return popupSettings.find((settings) => settings.target === target)!;
	};
</script>

{#if popupSettings.length > 0}
	{#each popupSettings as settings}
		{#if getPermissionsByTarget(settings.target)}
			<div data-popup={settings.target} class="p-2 bg-surface-300 rounded-md">
				<span>Manage: {getPermissionsByTarget(settings.target)?.manage ? '✅️' : '❌️'}</span><br />
				<span>Read: {getPermissionsByTarget(settings.target)?.read ? '✅️' : '❌️'}</span><br />
				<span>Change: {getPermissionsByTarget(settings.target)?.change ? '✅️' : '❌️'}</span><br />
				<span>Delete: {getPermissionsByTarget(settings.target)?.delete ? '✅️' : '❌️'}</span><br />
				<span>Create: {getPermissionsByTarget(settings.target)?.create ? '✅️' : '❌️'}</span><br />
				<span
					>Manage Download Share: {getPermissionsByTarget(settings.target)?.manageDownloadShare
						? '✅️'
						: '❌️'}</span
				><br />
				<span
					>Manage Upload Share: {getPermissionsByTarget(settings.target)?.manageUploadShare
						? '✅️'
						: '❌️'}</span
				><br />
				<span
					>Read Recycle Bin: {getPermissionsByTarget(settings.target)?.readRecycleBin
						? '✅️'
						: '❌️'}</span
				><br />
				<span
					>Restore Recycle Bin: {getPermissionsByTarget(settings.target)?.restoreRecycleBin
						? '✅️'
						: '❌️'}</span
				><br />
				<span
					>Delete Recycle Bin: {getPermissionsByTarget(settings.target)?.deleteRecycleBin
						? '✅️'
						: '❌️'}</span
				>
			</div>
		{/if}
	{/each}
{/if}

<div class="table-container">
	<table class="table table-hover">
		<thead>
			<tr>
				<th>Login</th>
				<th>First name</th>
				<th>Last name</th>
				<th>Permissions</th>
			</tr>
		</thead>
		<tbody>
			{#each userPermissions as row}
				<tr>
					<td>
						{row.userLogin}
					</td>
					<td>{row.userFirstName}</td>
					<td>{row.userLastName}</td>
					<td>
						<div use:popup={findSettingsByTarget(getPopupTarget(row))}>
							{displayPermissionsTemplate(row.permissions)}
						</div>
					</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>
