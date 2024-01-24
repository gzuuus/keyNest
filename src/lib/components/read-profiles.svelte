<script lang="ts">
	import ProfileIcon from '$lib/resources/icons/profile-icon.svelte';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import BinIcon from '$lib/resources/icons/bin-icon.svelte';
	import { deleteFile, getRootbyColumnAndValue, listFiles } from '$lib/resources/helpers';
	import { getToastStore } from '@skeletonlabs/skeleton';
	import { appContextStore } from '$lib/stores/stores';
	const toastStore = getToastStore();

	async function handleDeleteFile(file: string): Promise<boolean> {
		const deleted = await deleteFile(file);
		if (deleted) {
			toastStore.trigger({
				message: 'Profile deleted',
				background: 'variant-filled-success'
			})
			return true;
		} else {
			toastStore.trigger({
				message: 'Error deleting profile',
				background: 'variant-filled-error'
			})
			return false;
		}
	}
	onMount(() => {
		listFiles();
	});
</script>

<div>
	{#if $appContextStore?.fileList}
			<section class=" grid place-content-center gap-1">
				<nav class="list-nav bg-surface-active-token p-4 rounded flex flex-col gap-1">
					<h2>Select profile</h2>
					<hr class="!border-t-2" />
					<ul class="flex flex-col gap-2">
						{#each $appContextStore.fileList as file }
							<li>
								<div class="input-group input-group-divider grid-cols-[auto_1fr_auto]">
									<button on:click={() => getRootbyColumnAndValue(file, 'name', file.slice(0, -3))}>
										<span class="badge bg-primary-500"><ProfileIcon size={22} /></span>
										<span class="flex-auto">{file.slice(0, -3)}</span>
									</button>
									<button class="btn btn-sm btn-error" on:click={() => handleDeleteFile(file)}
										><BinIcon size={16} /></button
									>
								</div>
							</li>
						{/each}
					</ul>
				</nav>
				<button class="btn btn-sm" on:click={() => goto('/create-profile')}>Create new profile</button
				>
			</section>
	{/if}
</div>
