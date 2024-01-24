<script lang="ts">
	import IdentityCard from '$lib/components/identity-card.svelte';
import { decrypt, derive_child_pub_from_xpub, insertDerivedChild, logOut, readDb } from '$lib/resources/helpers';
	import { appContextStore, currentProfile, derivedIdentitiesStore } from '$lib/stores/stores';
	import type { ProfileInterface } from '$lib/types/interfaces';
	import { getToastStore } from '@skeletonlabs/skeleton';
	import { nip19 } from 'nostr-tools';
	import { onDestroy, onMount } from 'svelte';
	const toastStore = getToastStore();

	let prvk: string | undefined
	let password: string | undefined
	let showLogin: boolean;

	$: if ($currentProfile) {
		showLogin = true;
		console.log("currentProfile", $currentProfile.name)
	}

	async function handleLogin() {
		try {
			prvk = await decrypt($currentProfile?.prvk, password!);
			toastStore.trigger({
				message: 'Logged in',
				background: 'variant-filled-success'

			})
			password = undefined;
		} catch (e) {
			toastStore.trigger({
				message: 'Incorrect password',
				background: 'variant-filled-error'

			})
		}

	}

	async function handleDerive(parentIdentity: ProfileInterface | undefined, dbname: string | undefined) {
		if (!parentIdentity || !dbname) return
		let derivedKey = await derive_child_pub_from_xpub(parentIdentity, dbname);
		derivedKey && readDb(dbname)
	}
	onMount(() => {
		$appContextStore?.currentDbname? readDb($appContextStore?.currentDbname) : console.log("no db")
	})

	onDestroy(() => {
		prvk = undefined
	})
</script>

{#if showLogin}
	<div class="break-words">
		<h1 class="h2">{$currentProfile?.name}</h1>
		{#if !prvk}
		<form class="flex flex-col gap-2" on:submit|preventDefault={handleLogin}>
		<input class="input" bind:value={password} id="password" type="password" placeholder="Password"/>
		<button type="submit" class="common-btn-sm-filled">Login</button>
		</form>
		{:else}
		<h2>{nip19.npubEncode($currentProfile?.hexpub??'')}</h2>
		<button
			class="common-btn-sm-filled"
			on:click={() => handleDerive($currentProfile, $appContextStore?.currentDbname)}>Derive</button
		>
		<button class="common-btn-sm-ghost-error" on:click={() => logOut()}>Log Out</button>
			{#if $derivedIdentitiesStore}
				{#each $derivedIdentitiesStore as value}
					<IdentityCard profile={value} />
				{/each}	
			{/if}
		{/if}
	</div>
{/if}
