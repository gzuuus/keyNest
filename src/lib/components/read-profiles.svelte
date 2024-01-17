<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'
	import { fileStore } from '$lib/stores/stores';
	import ProfileIcon from '$lib/resources/icons/profile-icon.svelte';
	import CreateProfile from './create-profile.svelte';
	import { goto } from '$app/navigation';
    let content: any;
    let fileList: string[];

    async function read(name: string) {
       content = await invoke('read_file', { name })
    }
    async function listFiles(): Promise<string[] | undefined> {
        fileList = await invoke('list_files')
        console.log(fileList)
        if (fileList!.length) {
            fileStore.set(fileList)
            return fileList
        } else {
            return undefined
        }
    }

</script>
<div>
{#await listFiles() then files }
    {#if files}
        <section class=" grid place-content-center gap-1">
            <nav class="list-nav bg-surface-active-token p-4 rounded flex flex-col gap-1">
                Select profile
                <hr class="!border-t-2" />
                <ul class="flex flex-col gap-2">
                    {#each files as file}
                    <li>
                        <button on:click={()=> read(file)}>
                            <span class="badge bg-primary-500"><ProfileIcon size={22}/></span>
                            <span class="flex-auto">{file}</span>
                        </button>
                    </li>
                    {/each}
                </ul>
            </nav>
        <button class="btn btn-sm" on:click={()=> goto('/create-profile')}>Create new profile</button>
        </section>
    {:else}
        <CreateProfile/>
    {/if}
{/await}
<!-- <section class=" whitespace-pre-wrap">    
    {#if content}
        <pre>{JSON.stringify(content, null, 2)}</pre>
    {/if}
</section> -->

</div>
