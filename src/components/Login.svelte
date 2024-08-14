<script lang="ts">
	import { addHttps, validateUrl } from '$lib/url';
	import { login, setUserAccount } from '../stores/auth';
	import { goto } from '$app/navigation';
	import { connect, initAuthCodeFlow } from '$lib/auth';
	import { getToastStore } from '@skeletonlabs/skeleton';
	import { createToastSettings } from '$lib/utils';
	import { ToastType } from '$lib/models';

	const toastStore = getToastStore();

	let url = '';
	let authCode = '';
	let isRefreshToken = false;

	enum LoginState {
		Init,
		WaitingForCode,
		LoggedIn
	}

	let loginState = LoginState.Init;

	const handleLogin = async () => {
		url = addHttps(url);

		if (!await validateUrl(url)) {
			console.log('Invalid url', url);
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
	};
</script>

{#if loginState === LoginState.Init}
	<div class="login-container flex items-center space-x-2">
		<input
			class="input"
			title="Input (text)"
			type="text"
			placeholder="DRACOON url"
			bind:value={url}
		/>

		<button class="btn variant-filled-primary" on:click={handleLogin}> Log In </button>
	</div>
{/if}

{#if loginState === LoginState.WaitingForCode}
	<div class="code-container flex items-center space-x-2">
		<input
			class="input"
			title="Input (text)"
			type="text"
			placeholder="Authorization code"
			bind:value={authCode}
		/>

		<button class="btn variant-filled-primary" on:click={finalizeLogin}> Enter code </button>
		<button class="btn variant-filled-warning" on:click={handleCancel}> Cancel </button>
	</div>
{/if}
