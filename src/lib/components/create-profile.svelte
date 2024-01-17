<script lang="ts">
	import AddFileIcon from "$lib/resources/icons/add-file-icon.svelte";
	import { FileDropzone } from "@skeletonlabs/skeleton";
</script>
<section class=" grid place-items-center">
    <FileDropzone name="files">
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