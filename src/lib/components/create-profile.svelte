<script lang="ts">
	import AddFileIcon from '$lib/resources/icons/add-file-icon.svelte';
	import type { ProfileInterface } from '$lib/types/interfaces';
	import { FileDropzone, focusTrap } from '@skeletonlabs/skeleton';
	import { getToastStore } from '@skeletonlabs/skeleton';
	import { goto } from '$app/navigation';
	import { generateSecretKey, getPublicKey, nip19 } from 'nostr-tools';
	import CloseIcon from '$lib/resources/icons/close-icon.svelte';
	import { calculateXpubFromSeed, encrypt, insertInDb, uint8ArrayTo32HexString } from '$lib/resources/helpers';
	import { Accordion, AccordionItem } from '@skeletonlabs/skeleton';
	import GearIcon from '$lib/resources/icons/gear-icon.svelte';
	import { appContextStore, currentProfile } from '$lib/stores/stores';

	interface CreateAccForm {
		name: string;
		pass: string;
		nsec?: string;
	}

	let formData: Partial<CreateAccForm> = {};

	let name: string;
	let pass: string;
	let nsec: string | undefined;

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
								const data: ProfileInterface = JSON.parse(fileContent);
								console.log(data);
								const writeFiles = ''//await writeFile(data.name, data);
								writeFiles
									? toastStore.trigger({
											message: 'Profile created',
											background: 'variant-filled-success'
										})
									: toastStore.trigger({
											message: 'Error creating profile',
											background: 'variant-filled-error'
										});
								goto('/');
							} catch (error) {
								console.error('Error writing file:', error);
								toastStore.trigger({
									message: 'Error creating profile, not a valid json',
									background: 'variant-filled-error'
								});
							}
						}
					};
					reader.readAsText(files[i]);
				} else {
					console.log('not json');
					toastStore.trigger({
						message: 'You should import a json file',
						background: 'variant-filled-error'
					});
				}
			}
		};

		processFilesAsync().catch((error) => {
			console.error('Error processing files:', error);
		});
	}
	async function handleSubmit(): Promise<void> {
		function validateFormData(): boolean {
        const requiredFields = ['name', 'pass'];
        for (const field of requiredFields) {
            if (!(field in formData)) {
                console.warn(`Missing required field: ${field}`);
                return false;
            }
        }
        return true;
    }
		function readonlyValue(value: unknown): unknown {
			return typeof value === 'object' ? { ...value } : value;
		}
		formData = {
			name,
			pass,
			nsec
		};
		console.log(formData)

		let insecurePrvk = nsec ? nip19.decode(nsec!).data as Uint8Array : generateSecretKey();
		let hexpub = getPublicKey(insecurePrvk);
		let encryptedNsec = await encrypt(uint8ArrayTo32HexString(insecurePrvk), pass);
		let xpub = await calculateXpubFromSeed(uint8ArrayTo32HexString(insecurePrvk));
		let extendedFormData: ProfileInterface = {
			name,
			hexpub: hexpub,
			xpub: xpub,
			prvk: encryptedNsec,
			level: 0,
			gap: 0,
			comments: "root"
		};
		
		if (!validateFormData()) return;
		try {
			const formValues = Object.fromEntries(
				Object.entries(extendedFormData).map(([key, val]) => [key, readonlyValue(val)])
			) as Readonly<Record<keyof ProfileInterface, unknown>>;
			
			let dbName = `${formData.name}.db`
			let craftFile = await insertInDb(dbName, formValues as ProfileInterface);
			 console.log(craftFile);
			createNew = false;
			appContextStore.set({ 
				fileList: [dbName],
				currentDbname: dbName,
				sessionPass: formData.pass, 
			});
			currentProfile.set(extendedFormData);
			goto('/p/account');
		} catch (error) {
			console.log('Error occurred while handling submission:', error);
		}
	}

	let createNew: boolean = false;
</script>

{#if !createNew}
	<section class=" grid place-items-center">
		<FileDropzone name="files" bind:files on:change={onChangeHandler}>
			<svelte:fragment slot="lead">
				<section class=" flex justify-center">
					<AddFileIcon size={50} />
				</section>
			</svelte:fragment>
			<svelte:fragment slot="message">
				<p>Drop a profile file here to open</p>
			</svelte:fragment>
			<svelte:fragment slot="meta">(Also click here to explore)</svelte:fragment>
		</FileDropzone>
		<p>or</p>
		<button class="common-btn-sm-filled" on:click={() => (createNew = true)}
			>Create new profile</button
		>
	</section>
{:else}
	<section class=" flex flex-col gap-1 items-end">
		<button class="common-btn-icon-ghost" on:click={() => (createNew = false)}
			><CloseIcon size={16} /></button
		>
		<form use:focusTrap={true} on:submit|preventDefault={handleSubmit} class="flex flex-col gap-2">
			<label class="label"
				>Name:
				<input class="input" type="text" placeholder="Enter Name" bind:value={name} required />
			</label>
			<label class="label"
				>Pass:
				<input
					class="input"
					type="password"
					placeholder="Enter Password"
					bind:value={pass}
					required
				/>
			</label>

			<Accordion>
				<AccordionItem>
					<svelte:fragment slot="lead"><GearIcon size={18} /></svelte:fragment>
					<svelte:fragment slot="summary">Advanced</svelte:fragment>
					<svelte:fragment slot="content">
						<label class="label"
						>Import nsec:
						<input
							class="input"
							type="password"
							placeholder="Enter Nsec"
							bind:value={nsec}
							required
						/>
					</svelte:fragment>
				</AccordionItem>
			</Accordion>

			<button class="common-btn-sm-filled w-fit" type="submit">Publish</button>
		</form>
	</section>
{/if}
