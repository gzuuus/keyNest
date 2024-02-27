<script lang="ts">
	import '../app.postcss';
	import {
		AppShell,
		AppRail,
		AppRailAnchor,
		Toast,
		Modal,

		AppBar

	} from '@skeletonlabs/skeleton';
	import HouseIcon from '$lib/resources/icons/house-icon.svelte';
	import ProfileIcon from '$lib/resources/icons/profile-icon.svelte';
	import { currentProfile, appContextStore } from '$lib/stores/stores';
	import { page } from '$app/stores';
	import { initializeStores } from '@skeletonlabs/skeleton';
	import { goto } from '$app/navigation';
	initializeStores();
</script>

<Toast />
<Modal />
<AppShell>
	<svelte:fragment slot="header">
		<AppBar padding="p-1">
			<button class="hover:variant-soft px-2" on:click={() => goto('/')}>Home</button>
			<button class="hover:variant-soft px-2" on:click={() => goto('/settings')}>Settings</button>
		</AppBar>
	</svelte:fragment>
	<svelte:fragment slot="sidebarLeft">
		{#if $appContextStore?.fileList?.length}
			<AppRail>
				<AppRailAnchor href="/" selected={$page.url.pathname == '/'}>
					<svelte:fragment slot="lead"><HouseIcon size={18} /></svelte:fragment>
					<span>Home</span>
				</AppRailAnchor>
				{#if $currentProfile}
					<AppRailAnchor href="/p/account" selected={$page.url.pathname == '/p/account'}>
						<svelte:fragment slot="lead"><ProfileIcon size={18} /></svelte:fragment>
						<span>Account</span>
					</AppRailAnchor>
				{/if}
			</AppRail>
		{/if}
	</svelte:fragment>
	<div class=" h-full">
		<slot />
	</div>
	<svelte:fragment slot="pageFooter">
		{$appContextStore?.sessionPass? `loged as ${$currentProfile?.name}`  : 'not logged in'}
	</svelte:fragment>
</AppShell>
