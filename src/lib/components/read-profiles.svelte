<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { currentProfile, fileStore } from '$lib/stores/stores';
	import ProfileIcon from '$lib/resources/icons/profile-icon.svelte';
	import { goto } from '$app/navigation';
	import type { ProfileJsonInterface, RootPInterface } from '$lib/types/profile-json-interface';
	import { onMount } from 'svelte';
	import BinIcon from '$lib/resources/icons/bin-icon.svelte';
	let content: RootPInterface;
	let fileList: string[] | undefined;

	async function read(name: string) {
		content = await invoke('read_file', { name });
		console.log(content);
		currentProfile.set(content);
	}

	async function deleteFile(fileName: string) {
		try {
			let deleteFile = await invoke('delete_file_by_name', { filename: fileName });
			if (deleteFile) listFiles();
		} catch (error) {
			console.log(error);
		}
	}
	async function listFiles(): Promise<string[] | undefined> {
		fileList = await invoke('list_files');
		console.log(fileList);
		if (fileList!.length) {
			fileStore.set(fileList);
			return fileList;
		} else if (fileList!.length == 0) {
			console.log('no files');
			goto('/create-profile');
			fileStore.set(undefined);
			return undefined;
		}
	}
	onMount(async () => {
		fileList = await listFiles();
		fileList ? goto('/') : goto('/create-profile');
	});
</script>

<div>
	{#if fileList}
		<section class=" grid place-content-center gap-1">
			<nav class="list-nav bg-surface-active-token p-4 rounded flex flex-col gap-1">
				<h2>Select profile</h2>
				<hr class="!border-t-2" />
				<ul class="flex flex-col gap-2">
					{#each fileList as file}
						<li>
							<div class="input-group input-group-divider grid-cols-[auto_1fr_auto]">
								<button on:click={() => read(file)}>
									<span class="badge bg-primary-500"><ProfileIcon size={22} /></span>
									<span class="flex-auto">{file}</span>
								</button>
								<button class="btn btn-sm btn-error" on:click={() => deleteFile(file)}
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
