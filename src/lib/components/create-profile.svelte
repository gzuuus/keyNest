<script lang="ts">
	import AddFileIcon from "$lib/resources/icons/add-file-icon.svelte";
	import type { ProfileJsonInterface, RootPInterface } from "$lib/types/profile-json-interface";
	import { FileDropzone, focusTrap } from "@skeletonlabs/skeleton";
    import { invoke } from '@tauri-apps/api/tauri'
    import { getToastStore } from '@skeletonlabs/skeleton';
	import { goto } from "$app/navigation";
    import { number, object, string } from 'zod';
    import { generateSecretKey, getPublicKey, nip19 } from "nostr-tools";
    import  CloseIcon from '$lib/resources/icons/close-icon.svelte';

    const scryptSchema = object({
        salt: string(),
        n: number(),
        r: number(),
        p: number()
    });

    export const rootPSchema = object({
        name: string(),
        npub: string(),
        pass: string(),
        xpub: string(),
        prvk: string(),
        level: number(),
        scrypt: scryptSchema,
        scope: number()
    });

    interface extendedRoot extends RootPInterface {
        pass: string;
    }

    let insecureNsec = generateSecretKey();
    let formData: Partial<extendedRoot> = {};

    let name: string;
    let pass: string;
    let npub: string = nip19.npubEncode(getPublicKey(insecureNsec));
    let xpub: string = 'xpub661MyMwAqRbcEeL4JhzVoecahueJ7kD6reQDSqsVkXuTay22563GhV8x762JKYf2LQtwkKdiunLLZYAWgajY8KtptHcCJ2zNBjszvbA4X4a';
    let prvk: string = nip19.nsecEncode(insecureNsec);
    let level: number = 0;
    let salt: string = '00000000000000000000000000000000';
    let n: number = 256;
    let r: number = 8;
    let p: number = 1;
    let scope: number = 0;

    const toastStore = getToastStore();

    let files: FileList;

    async function onChangeHandler() {
    const processFilesAsync = async () => {
        for (let i = 0; i < files.length; i++) {
            if (files[i].type === 'application/json') {
            const reader = new FileReader();
            reader.onload = async (event) => {
                const fileContent = event.target?.result;
                if (typeof fileContent === 'string') {
                try {
                    const data: ProfileJsonInterface = JSON.parse(fileContent);
                    console.log(data);
                    const writeFiles = await writeFile(data.user, data);
                    writeFiles ? toastStore.trigger({ message: 'Profile created', background: 'variant-filled-success'}) : toastStore.trigger({ message: 'Error creating profile', background: 'variant-filled-error'});
                    goto('/');
                } catch (error) {
                    console.error("Error writing file:", error);
                    toastStore.trigger({ message: 'Error creating profile, not a valid json', background: 'variant-filled-error'})
                }
                }
            };
            reader.readAsText(files[i]);
            } else {
            console.log('not json');
            toastStore.trigger({ message: 'You should import a json file', background: 'variant-filled-error'})
            }
        }
    };

    processFilesAsync().catch((error) => {
        console.error("Error processing files:", error);
    });

    }
    //
    let data = {"name": "profile"}
    async function writeFile(name: string, data:any): Promise<boolean> {
      return await invoke('write_json', { name , data })
    }
    //
    
    async function handleSubmit(): Promise<void> {
        function validateFormData(): boolean {
        return !!formData && Object.values(formData).every(Boolean);
        }
        function readonlyValue(value: unknown): unknown {
            return typeof value === "object" ? {...value} : value;
        }
        console.log(formData);
        formData = {
            name,
            pass,
            npub,
            xpub,
            prvk,
            level,
            scrypt: {
            n,
            r,
            p,
            salt,
            },
            scope
        };

        if (!validateFormData()) return;
        console.log(formData);
        try {
            const formValues = Object.fromEntries(Object.entries(formData).map(([key, val]) => [key, readonlyValue(val)])) as Readonly<Record<keyof RootPInterface, unknown>>;
            console.log(formValues);
            let craftFile = writeFile(formData.name! , formValues);
            console.log(craftFile)
            createNew = false;
            goto('/');
        } catch (error) {
            console.log("Error occurred while handling submission:", error);
        }
    }

    let createNew: boolean = false;

</script>
{#if !createNew}
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
    <button class="common-btn-sm-filled" on:click="{()=> createNew = true}">Create new profile</button>
</section>
{:else}
<section class=" flex flex-col gap-1 items-end">
    <button class="common-btn-icon-ghost" on:click="{()=> createNew = false}"><CloseIcon size={16}/></button>
    <form use:focusTrap={true} on:submit|preventDefault={handleSubmit}>
        <label class="label">Name:
          <input class="input" type="text" placeholder="Enter Name" bind:value={name} required />
        </label>
        <label class="label">Pass:
            <input class="input" type="password" placeholder="Enter Name" bind:value={pass} required />
        </label>
        <label class="label">npub:
          <input class="input" type="text" placeholder="Enter npub" bind:value={npub} required />
        </label>
        <label class="label">xpub:
          <input class="input" type="text" placeholder="Enter xpub" bind:value={xpub} required />
        </label>
        <label class="label">prvk:
          <input class="input" type="text" placeholder="Enter prvk" bind:value={prvk} required />
        </label>
        <label class="label">Level:
          <input class="input" type="number" min="0" max="100" step="1" bind:value={level} required />
        </label>
        <label class="label">Salt:
          <input class="input" type="text" placeholder="Enter Salt" bind:value={salt} required />
        </label>
        <label class="label">N:
          <input class="input" type="number" min="0" max="256" step="1" bind:value={n} required />
        </label>
        <label class="label">R:
          <input class="input" type="number" min="0" max="256" step="1" bind:value={r} required />
        </label>
        <label class="label">P:
          <input class="input" type="number" min="0" max="256" step="1" bind:value={p} required />
        </label>
        <label class="label">Scope:
          <input class="input" type="number" min="0" max="100" step="1" bind:value={scope} required />
        </label>
        
        <button class="common-btn-sm-filled" type="submit">Publish</button>
      </form>
      
</section>
{/if}