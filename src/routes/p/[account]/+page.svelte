<script lang="ts">
	import { decrypt, derive_child_pub_from_xpub, insertDerivedChild, logOut } from '$lib/resources/helpers';
	import { currentProfile } from '$lib/stores/stores';
	import { getToastStore } from '@skeletonlabs/skeleton';
	const toastStore = getToastStore();

	let prvk: string | undefined
	let password: string
	let showLogin: boolean;

	$: if ($currentProfile) {
		showLogin = true;
		console.log("currentProfile", $currentProfile.name)
	}

	async function handleLogin() {
		try {
			prvk = await decrypt($currentProfile?.prvk, password);
			toastStore.trigger({
				message: 'Logged in',
				background: 'variant-filled-success'

			})
			password = '';
		} catch (e) {
			toastStore.trigger({
				message: 'Incorrect password',
				background: 'variant-filled-error'

			})
		}

	}

	async function handleDerive() {
		let derivedKey = await derive_child_pub_from_xpub($currentProfile?.xpub!, 0);
		let insertKey = await insertDerivedChild(`${$currentProfile?.name!}.db`, derivedKey);
		console.log(derivedKey, insertKey);
	}
</script>

{#if showLogin}
	<div class="break-words">
		<h1 class="h2">{$currentProfile?.name}</h1>
		{#if !prvk}
		<form class="flex flex-col gap-2" on:submit|preventDefault={() => console.log('')}>
		<input class="input" bind:value={password} id="password" type="password" placeholder="Password"/>
		<button type="submit" class="common-btn-sm-filled" on:click={() => handleLogin()}>Login</button>
		</form>
		{:else}
		<h2>{$currentProfile?.npub}</h2>
		<button
			class="common-btn-sm-filled"
			on:click={() => handleDerive()}>Derive</button
		>
		<button class="common-btn-sm-ghost-error" on:click={() => logOut()}>Log Out</button>
		{/if}
	</div>
{/if}
