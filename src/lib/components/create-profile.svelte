<script lang="ts">
	import AddFileIcon from "$lib/resources/icons/add-file-icon.svelte";
	import { FileDropzone } from "@skeletonlabs/skeleton";
    import { invoke } from '@tauri-apps/api/tauri'

    let files: FileList;

    function onChangeHandler(): void {
    for (let i = 0; i < files.length; i++) {
            if (files[i].type === 'application/json') {
                console.log(files[i]);
            } else {
                console.log('not json');
            }
        }
    }
    let data = {"name": "profile"}
    async function write(name: string, data:any) {
       invoke('write_json', { name , data })
    }
</script>
<button class="common-btn-sm-filled" on:click="{()=> write('profile', data)}">Create new profile</button>
<section class=" grid place-items-center">
    <FileDropzone name="files" bind:files={files} on:change={onChangeHandler}>
            <svelte:fragment slot="lead">
                <section class=" flex justify-center">
                    <AddFileIcon size={50}/>
                </section>
            </svelte:fragment>
            <svelte:fragment slot="message">
                <p>Drop a profile file here to open</p>

            </svelte:fragment>
            <svelte:fragment slot="meta">
                (Also click here to explore)
            </svelte:fragment>
    </FileDropzone>
    <p>or</p>
    <button class="common-btn-sm-filled" on:click="{()=> console.log('hello')}">Create new profile</button>
    </section>