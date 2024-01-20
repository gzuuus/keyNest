<script lang="ts">
	import ProfileIcon from '$lib/resources/icons/profile-icon.svelte';
	import { goto } from '$app/navigation';
	import { onMount } from 'svelte';
	import BinIcon from '$lib/resources/icons/bin-icon.svelte';
	import { deleteFile, listFiles, read } from '$lib/resources/helpers';
	let fileList: string[] | undefined;

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
