<script lang="ts">
	import { addHttps, validateUrl } from '$lib/url';
	import { login, setUserAccount } from '../stores/auth';
	import { goto } from '$app/navigation';
	import { connect, initAuthCodeFlow } from '$lib/auth';
	import { getToastStore } from '@skeletonlabs/skeleton';
	import { createToastSettings } from '$lib/utils';
	import { ToastType } from '$lib/models';
	import { onMount } from 'svelte';

	// Focus directive for accessibility
	const focus = (node: HTMLElement) => {
		// Focus the node
		node.focus();
		return {};
	};

	const toastStore = getToastStore();

	let url = '';
	let authCode = '';
	let isRefreshToken = false;
	let urlInput: HTMLInputElement;

	enum LoginState {
		Init,
		WaitingForCode,
		LoggedIn
	}

	let loginState = LoginState.Init;

	// Focus the input field when component mounts
	onMount(() => {
		if (urlInput) {
			urlInput.focus();
		}
	});

	const handleLogin = async () => {
		url = addHttps(url);

		if (!await validateUrl(url)) {
			const errrorToast = createToastSettings(`Invalid url (${url})`, ToastType.Error);
			toastStore.trigger(errrorToast);
			loginState = LoginState.Init;
			return;
		}

		try {
			isRefreshToken = await initAuthCodeFlow(url);
		} catch (err) {
			const errrorToast = createToastSettings(`Login failed (${err})`, ToastType.Error);
			toastStore.trigger(errrorToast);
			loginState = LoginState.Init;
			return;
		}

		if (!isRefreshToken) {
			loginState = LoginState.WaitingForCode;
			return;
		}

		finalizeLogin();
	};

	const finalizeLogin = async () => {
		try {
			const userAccount = await connect(isRefreshToken, authCode);
			setUserAccount(userAccount);
		} catch (err) {
			const errrorToast = createToastSettings(`Login failed (${err})`, ToastType.Error);
			toastStore.trigger(errrorToast);
			loginState = LoginState.Init;
			return;
		}

		loginState = LoginState.LoggedIn;
		login();

		goto('/');
	};

	const handleCancel = () => {
		loginState = LoginState.Init;
		// Re-focus the URL input after canceling
		setTimeout(() => {
			if (urlInput) {
				urlInput.focus();
			}
		}, 0);
	};

	// Handle Enter key for login
	const handleKeydown = (e: KeyboardEvent) => {
		if (e.key === 'Enter') {
			handleLogin();
		}
	};

	// Handle Enter key for code input
	const handleCodeKeydown = (e: KeyboardEvent) => {
		if (e.key === 'Enter') {
			finalizeLogin();
		}
	};
</script>

<div class="card p-4 w-full">
	{#if loginState === LoginState.Init}
		<div class="login-container grid grid-cols-1 sm:grid-cols-[1fr,auto] gap-2 w-full">
			<input
				class="input"
				title="Input (text)"
				type="text"
				placeholder="DRACOON url"
				bind:value={url}
				bind:this={urlInput}
				on:keydown={handleKeydown}
			/>

			<button class="btn variant-filled-primary h-10" on:click={handleLogin}> Log In </button>
		</div>
	{/if}

	{#if loginState === LoginState.WaitingForCode}
		<div class="code-container grid grid-cols-1 sm:grid-cols-[1fr,auto,auto] gap-2 w-full">
			<input
				class="input"
				title="Input (text)"
				type="text"
				placeholder="Authorization code"
				bind:value={authCode}
				on:keydown={handleCodeKeydown}
				use:focus
			/>

			<button class="btn variant-filled-primary h-10" on:click={finalizeLogin}> Enter code </button>
			<button class="btn variant-outline-warning h-10" on:click={handleCancel}> Cancel </button>
		</div>
	{/if}
</div>
