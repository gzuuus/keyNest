<script lang="ts">
	import '../app.postcss';
	import {
		AppShell,
		AppRail,
		AppRailAnchor,
		Toast,
		Modal
	} from '@skeletonlabs/skeleton';
	import ndk from '$lib/stores/provider';
	import HouseIcon from '$lib/resources/icons/house-icon.svelte';
	import ProfileIcon from '$lib/resources/icons/profile-icon.svelte';
	import { currentProfile, appContextStore } from '$lib/stores/stores';
	import { page } from '$app/stores';
	import { initializeStores } from '@skeletonlabs/skeleton';
	import { goto } from '$app/navigation';
	initializeStores();
	
	currentProfile.subscribe(setProfile);
	async function setProfile() {
		console.log($currentProfile);
		goto(`/p/account`);
	}
	// $: console.log('Reading appContextStore', $appContextStore);
	// Highlight JS
	// import hljs from 'highlight.js/lib/core';
	// import 'highlight.js/styles/github-dark.css';
	// import { storeHighlightJs } from '@skeletonlabs/skeleton';
	// import xml from 'highlight.js/lib/languages/xml'; // for HTML
	// import css from 'highlight.js/lib/languages/css';
	// import javascript from 'highlight.js/lib/languages/javascript';
	// import typescript from 'highlight.js/lib/languages/typescript';

	// hljs.registerLanguage('xml', xml); // for HTML
	// hljs.registerLanguage('css', css);
	// hljs.registerLanguage('javascript', javascript);
	// hljs.registerLanguage('typescript', typescript);
	// storeHighlightJs.set(hljs);

	// Floating UI for Popups
	// import { computePosition, autoUpdate, flip, shift, offset, arrow, size } from '@floating-ui/dom';
	// import { storePopup } from '@skeletonlabs/skeleton';

	// storePopup.set({ computePosition, autoUpdate, flip, shift, offset, arrow });
</script>

<Toast />
<Modal />
<AppShell>
	<svelte:fragment slot="header">
		<button on:click={() => goto('/')}>Home</button>
		<button on:click={() => goto('/test')}>test</button>
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
	<div class=" grid place-content-center h-full w-auto">
		<slot />
	</div>
	<svelte:fragment slot="pageFooter">{$page.url.href}</svelte:fragment>
</AppShell>
