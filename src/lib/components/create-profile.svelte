<script lang="ts">
	import AddFileIcon from '$lib/resources/icons/add-file-icon.svelte';
	import type { ProfileInterface } from '$lib/types/interfaces';
	import { FileDropzone, focusTrap } from '@skeletonlabs/skeleton';
	import { getToastStore } from '@skeletonlabs/skeleton';
	import { goto } from '$app/navigation';
	import { generateSecretKey, getPublicKey, nip19 } from 'nostr-tools';
	import CloseIcon from '$lib/resources/icons/close-icon.svelte';
	import { calculateXpubFromSeed, encrypt, generate_id, hexStringToUint8Array, insertInDb, mnemonics_from_seed, seed_from_mnemonics, uint8ArrayTo32HexString } from '$lib/resources/helpers';
	import { Accordion, AccordionItem } from '@skeletonlabs/skeleton';
	import GearIcon from '$lib/resources/icons/gear-icon.svelte';
	import * as zod from 'zod';
	import { appContextStore, currentProfile } from '$lib/stores/stores';
	import ProfileIcon from '$lib/resources/icons/profile-icon.svelte';
	
	const toastStore = getToastStore();
	let createNew: boolean = false;
	let isValidForm: boolean = false;

	const createAccFormSchema = zod.object({
		name: zod.string().max(20, 'Name must be at most 20 characters long'),
		pass: zod.string().min(6, 'Password must be at least 6 characters long'),
		nsec: zod.string().optional(),
		mnemonics: zod.string().optional()
	}).strict();

	type FormDataable = zod.infer<typeof createAccFormSchema>;

	let formData: FormDataable;

	let name: string;
	let pass: string;
	let nsec: string | undefined;
	let mnemonics: string | undefined;

	interface ValidationErrors {
		name?: string;
		pass?: string;
		nsec?: string;
		mnemonics?: string;
	}
	let validationErrors:ValidationErrors = {};

	async function handleSubmit(): Promise<void> {
		try {
			formData = createAccFormSchema.parse({
				name,
				pass,
				nsec,
				mnemonics
			});

			let insecurePrvk;
			if (nsec) {
				if (!nsec.startsWith('nsec')) {
					insecurePrvk = hexStringToUint8Array(nsec!);
				} else {
					insecurePrvk = nip19.decode(nsec!).data as Uint8Array;
				}
			} else if (mnemonics) {
				insecurePrvk = await seed_from_mnemonics(mnemonics);
				insecurePrvk = hexStringToUint8Array(insecurePrvk!);
			} else {
				const newId = await generate_id();
				insecurePrvk = nip19.decode(newId.seed).data as Uint8Array;
				mnemonics = newId.mnemonic;
			}
			const hexpub = getPublicKey(insecurePrvk);
			const encryptedNsec = await encrypt(uint8ArrayTo32HexString(insecurePrvk), pass);
			const xpub = await calculateXpubFromSeed(uint8ArrayTo32HexString(insecurePrvk));
			const extendedFormData: ProfileInterface = {
				name,
				hexpub: hexpub,
				xpub: xpub,
				prvk: encryptedNsec,
				level: 0,
				gap: 0,
				comments: "root"
			};

			if (!createAccFormSchema.safeParse(formData).success) throw new Error("Invalid form data");

			try {
				const formValues = Object.fromEntries(
					Object.entries(extendedFormData).map(([key, val]) => [key, val])
				) as Readonly<Record<keyof ProfileInterface, unknown>>;

				let dbName = `${formData.name}.db`;
				await insertInDb(dbName, formValues as ProfileInterface);
				
				appContextStore.set({
					fileList: undefined,
					currentDbname: dbName,
					sessionPass: formData.pass,  
				});
				currentProfile.set(extendedFormData);
				isValidForm = true;

			} catch (error) {
				console.log('Error occurred while handling submission:', error);
			}
		} catch (error: any ) {
			console.log('Validation or other errors occurred:', error);
			validationErrors = error.formErrors.fieldErrors;
			for (let error of parseErrors(validationErrors)) {
				toastStore.trigger({
					message: error,
					background: 'variant-filled-error'
				})
			}
		}
	}

	function handleComplete() {
		appContextStore.update((value)=>{
			return {
				fileList: [value?.currentDbname!],
				currentDbname: value?.currentDbname,
				sessionPass: value?.sessionPass,
			}
		})
		console.log($appContextStore)
		createNew = false;
		goto('/p/account');
	}

	function parseErrors(errors: ValidationErrors): string[] {
		return Object.entries(errors).map(([key, value]) => `${key}: ${value}`);
	}

	function parseMnemonics(mnemonics: string): string[] {
		return mnemonics.split(' ');
	}
	
</script>

{#if !createNew}
	<section class=" grid place-items-center gap-4">
		<ProfileIcon size={64} />
		<button class="common-btn-sm-filled" on:click={() => (createNew = true)}
			>Create new profile</button
		>
	</section>
{:else}
	
	<section class=" flex flex-col gap-1 items-end">
		{#if !isValidForm}
		<button class="common-btn-icon-ghost" on:click={() => (createNew = false)}
			><CloseIcon size={16} /></button
		>
		<form use:focusTrap={true} on:submit|preventDefault={handleSubmit} class="flex flex-col gap-2">
			<label class="label"
				>Name:
				<input class="input" class:input-error={validationErrors.name} type="text" placeholder="Enter Name" bind:value={name} required />
			</label>
			<label class="label"
				>Pass:
				<input
					class="input"
					class:input-error={validationErrors.pass}
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
							class:input-error={validationErrors.nsec}
							type="password"
							placeholder="Enter Nsec"
							bind:value={nsec}
						/>
						<label class="label"
						>Import mnemonics:
						<input
							class="input"
							type="text"
							placeholder="Enter mnemonics"
							bind:value={mnemonics}
						/>
					</svelte:fragment>
				</AccordionItem>
			</Accordion>

			<button class="common-btn-sm-filled w-fit" type="submit">Publish</button>
		</form>
		{:else}
		<div class=" card flex flex-col gap-1 px-4 py-2">
			<h2 class="h3">Profile Created</h2>
			<p>Please take a moment to check your profile and write down your seed phrase.</p>
			<hr/>
			<p>Name: {name}</p>
			<p>Seed phrase:</p>
			<ol class="list columns-2">
				{#each parseMnemonics(mnemonics ? mnemonics : '') as mnemonic, index}
				<li class="variant-soft p-2 rounded">
					<span>{index+1}.</span>
					<span class="flex-auto">{mnemonic}</span>
				</li>
				{/each}
			</ol>
			<button class="common-btn-sm-filled w-fit" on:click={handleComplete}>Done</button>
		</div>
		{/if}
	</section>
{/if}
