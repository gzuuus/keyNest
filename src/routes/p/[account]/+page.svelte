<script lang="ts">
	import IdentityCard from '$lib/components/identity-card.svelte';
	import ParentCard from '$lib/components/parent-card.svelte';
	import { decrypt, derive_child_from_seed_and_insert, derive_child_pub_from_xpub_and_insert, insertDerivedChild, logOut, readDb } from '$lib/resources/helpers';
	import { appContextStore, currentProfile, derivedIdentitiesStore } from '$lib/stores/stores';
	import type { ProfileInterface } from '$lib/types/interfaces';
	import { focusTrap, getToastStore } from '@skeletonlabs/skeleton';
	import { nip19 } from 'nostr-tools';
	import { onMount } from 'svelte';
	const toastStore = getToastStore();

	let prvk: string | undefined
	let password: string | undefined
	let showLogin: boolean

	async function handleLogin() {
		try {
			prvk = await decrypt($currentProfile?.prvk, password!);
			if (!$appContextStore?.sessionPass) {
				appContextStore.update((value)=>{
					return {
						fileList: value?.fileList,
						currentDbname: value?.currentDbname, 
						sessionPass: password}
				})
			toastStore.trigger({
				message: 'Logged in',
				background: 'variant-filled-success'
			})
			}
			showLogin = false
		} catch (e) {
			toastStore.trigger({
				message: 'Incorrect password',
				background: 'variant-filled-error'

			})
		}

	}

	async function handleDerive(parentIdentity: ProfileInterface | undefined, dbname: string | undefined, password: string | undefined) {
		console.log('parent', parentIdentity, dbname)
		if (!parentIdentity || !dbname || !password) return
		let derivedKey = await derive_child_from_seed_and_insert(parentIdentity, password!, dbname);
		derivedKey && readDb(dbname)
	}
	onMount(() => {
		password = $appContextStore?.sessionPass ? $appContextStore?.sessionPass : undefined
		if (password && !prvk) {
			handleLogin()
			showLogin = false
		} else if (password && prvk) {
			showLogin = false
		} else {
			showLogin = true;
		}
		$appContextStore?.currentDbname? readDb($appContextStore?.currentDbname) : console.log("no db")
	})
</script>

{#if showLogin}
	<h1 class="h2">{$currentProfile?.name}</h1>
	<form use:focusTrap={true} on:submit|preventDefault={handleLogin} class="flex flex-col gap-2" >
		<input class="input" bind:value={password} id="password" type="password" placeholder="Password"/>
		<button type="submit" class="common-btn-sm-filled w-fit">Login</button>
	</form>
{:else}
	<div class=" break-words grid grid-cols-1 gap-4">
		<section>
			{#if $currentProfile}
			<ParentCard profile={$currentProfile} />
			{/if}
			<button
				class="common-btn-sm-filled"
				on:click={() => handleDerive($currentProfile, $appContextStore?.currentDbname, password)}>Derive</button
			>
			<button class="common-btn-sm-ghost-error" on:click={() => logOut()}>Log Out</button>
		</section>
		<hr/>
		<section class=" flex flex-col gap-2">
			{#if $derivedIdentitiesStore}
				{#each $derivedIdentitiesStore as value}
					<IdentityCard profile={value} />
				{/each}	
			{/if}
		</section>
	</div>
{/if}
