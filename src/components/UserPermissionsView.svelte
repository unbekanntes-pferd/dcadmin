<script lang="ts">
	import type { NodePermissions, NodePermissionsList } from '$lib/permissions/models';
	import { toReadableSize } from '$lib/utils';
	import { popup, type PopupSettings } from '@skeletonlabs/skeleton';

	export let permissionsList: NodePermissionsList[];

	enum PermissionsTemplate {
		RoomAdministator = 'Room Administrator',
		Edit = 'Edit',
		Read = 'Read',
		Custom = 'Custom',
		None = 'None'
	}

	const displayPermissionsTemplate = (perms: NodePermissions): PermissionsTemplate => {
		if (!perms) {
			return PermissionsTemplate.None;
		}

		if (
			perms.manage &&
			perms.read &&
			perms.change &&
			perms.delete &&
			perms.create &&
			perms.manageDownloadShare &&
			perms.manageUploadShare &&
			perms.readRecycleBin &&
			perms.restoreRecycleBin &&
			perms.deleteRecycleBin
		) {
			return PermissionsTemplate.RoomAdministator;
		}

		if (
			perms.read &&
			perms.change &&
			perms.delete &&
			perms.create &&
			perms.manageDownloadShare &&
			perms.manageUploadShare &&
			perms.readRecycleBin &&
			perms.restoreRecycleBin
		) {
			return PermissionsTemplate.Edit;
		}

		if (perms.read && perms.manageDownloadShare) {
			return PermissionsTemplate.Read;
		}

		return PermissionsTemplate.Custom;
	};

	$: popupSettings = permissionsList.map((permissions) => {
		return {
			event: 'hover',
			target: getPopupTarget(permissions),
			placement: 'bottom'
		} as PopupSettings;
	});

	const getPopupTarget: (row: NodePermissionsList) => string = (row) => {
		return `${row.nodeId}-${row.userPermissions[0].userId}`;
	};

	const findSettingsByTarget: (target: string) => PopupSettings = (target) => {
		return popupSettings.find((settings) => settings.target === target)!;
	};

	const getPermissionsByTarget: (target: string) => NodePermissions | undefined = (target) => {
		const row = permissionsList.find((row) => getPopupTarget(row) === target);
		return row?.userPermissions[0].permissions;
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

<div class="overflow-x-auto space-y-2 h-full">
	<div class="table-container">
		<table class="table table-hover">
			<thead>
				<tr>
					<th>Name</th>
					<th>Path</th>
					<th>Size</th>
					<th>Quota</th>
					<th>Permissions</th>
					<th>Encrypted</th>
				</tr>
			</thead>
			<tbody>
				{#each permissionsList as row}
					<tr>
						<td>{row.nodeName}</td>
						<td>{row.nodeParentPath}</td>
						<td>{row.nodeSize ? toReadableSize(row.nodeSize) : 0}</td>
						<td>{row.nodeQuota ? toReadableSize(row.nodeQuota) : 'N/A'}</td>
						<td>
							{#if row.userPermissions.length > 0}
								<div use:popup={findSettingsByTarget(getPopupTarget(row))}>
									{displayPermissionsTemplate(row.userPermissions[0].permissions)}
								</div>
							{:else}
								{PermissionsTemplate.None}
							{/if}
						</td>
						<td>{row.nodeIsEncrypted ? '✅️' : '❌'}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
</div>

<style lang="css">
	td {
		padding: 1rem;
	}

	table thead {
		position: sticky;
		top: 0;
		z-index: 1;
	}
</style>
