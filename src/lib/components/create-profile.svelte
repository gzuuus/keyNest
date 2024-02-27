<script lang="ts">
	import AddFileIcon from '$lib/resources/icons/add-file-icon.svelte';
	import type { ProfileInterface } from '$lib/types/interfaces';
	import { FileDropzone, focusTrap } from '@skeletonlabs/skeleton';
	import { getToastStore } from '@skeletonlabs/skeleton';
	import { goto } from '$app/navigation';
	import { generateSecretKey, getPublicKey, nip19 } from 'nostr-tools';
	import CloseIcon from '$lib/resources/icons/close-icon.svelte';
	import { calculateXpubFromSeed, encrypt, generate_id, hexStringToUint8Array, insertInDb, mine_id, mnemonics_from_seed, num_cpus, seed_from_mnemonics, truncateString, uint8ArrayTo32HexString } from '$lib/resources/helpers';
	import { Accordion, AccordionItem } from '@skeletonlabs/skeleton';
	import GearIcon from '$lib/resources/icons/gear-icon.svelte';
	import * as zod from 'zod';
	import { appContextStore, currentProfile } from '$lib/stores/stores';
	import ProfileIcon from '$lib/resources/icons/profile-icon.svelte';
	import { InputChip } from '@skeletonlabs/skeleton';
	import { BECH32_CHARS } from '$lib/resources/constants';
	import { onMount } from 'svelte';
	import MiningIcon from '$lib/resources/icons/mining-icon.svelte';
	
	let createNew: boolean = false;

	const toastStore = getToastStore();
	let isValidForm: boolean = false;

	const createAccFormSchema = zod.object({
		name: zod.string().max(20, 'Name must be at most 20 characters long'),
		pass: zod.string().min(6, 'Password must be at least 6 characters long'),
		nsec: zod.string().optional(),
		mnemonics: zod.string().optional(),
		log_n: zod.number().optional()
	}).strict();

	type FormDataable = zod.infer<typeof createAccFormSchema>;

	let formData: FormDataable;

	let name: string;
	let pass: string;
	let nsec: string | undefined;
	let mnemonics: string | undefined;
	let log_n: number | undefined

	interface ValidationErrors {
		name?: string;
		pass?: string;
		nsec?: string;
		mnemonics?: string;
		log_n? : number
	}
	let validationErrors:ValidationErrors = {};

	async function handleSubmit(): Promise<void> {
		try {
			formData = createAccFormSchema.parse({
				name,
				pass,
				nsec,
				mnemonics,
				log_n
			});
			console.log(formData);
			let insecurePrvk;
			if (nsec) {
				if (!nsec.startsWith('nsec')) {
					insecurePrvk = hexStringToUint8Array(nsec!);
					mnemonics = await mnemonics_from_seed(nsec!);
				} else {
					insecurePrvk = nip19.decode(nsec!).data as Uint8Array;
					mnemonics = await mnemonics_from_seed(uint8ArrayTo32HexString(insecurePrvk));
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
			const encryptedNsec = await encrypt(uint8ArrayTo32HexString(insecurePrvk), pass, log_n);
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

	let prefixList: string[] = [];
	let maxCpus: number = 0
	let cpusToUse: number = 1;
	let mined_id: string[] = [];

	async function mine(prefixes: string[], cores: number){
		let toastId = toastStore.trigger({
			message: 'Mining',
			background: ' variant-filled-primary',
			autohide: false
		})
			console.log("Mining", prefixes, cores)
			mined_id = await mine_id(prefixes, cores);
			nsec = mined_id[1];
			toastStore.close(toastId);
		}

	async function count_cpus(): Promise<number> {
		maxCpus = await num_cpus();
		cpusToUse = maxCpus;
		return maxCpus
	}

	function isValidPrefix(value: string): boolean {
		let missingChars = '';
		for (const char of value) {
			if (!BECH32_CHARS.includes(char)) {
			missingChars += char;
			}
		}
		if (missingChars !== '') {
			toastStore.trigger({
					message: `The following characters are not valid: ${missingChars}`,
					background: 'variant-filled-error'
			})
		}
		return !missingChars || missingChars.length === 0;
	}
	onMount(async () => {
		count_cpus();
	})
</script>

{#if !createNew}
	<section class=" grid place-items-center gap-4">
		<ProfileIcon size={64} />
		<button class="common-btn-sm-filled" on:click={() => (createNew = true)}
			>Create new profile</button
		>
	</section>
{:else}
	
	<section class=" flex flex-col gap-1 items-end max-w-sm">
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
						<label class="label"
						>Log_n: <small class=" opacity-40">the higher the safest, but takes longer to compute</small>
						<input
							class="input"
							type="number"
							placeholder="16, 18, 22, ...(recommended >= 16)"
							bind:value={log_n}
						/>
					</svelte:fragment>
				</AccordionItem>
				<AccordionItem>
					<svelte:fragment slot="lead"><MiningIcon size={18} /></svelte:fragment>
					<svelte:fragment slot="summary">Mine id</svelte:fragment>
					<svelte:fragment slot="content">
						<p>Add a prefixes to mine</p>
						<small>Prefixes only can cointain valid bech32 characters</small>
						<InputChip bind:value={prefixList} name="chips" placeholder="Enter prefixes...(intro to push)" validation={isValidPrefix}/>	
						{#if prefixList.length}
							<label class="label"
							> Cores to use?:
							{cpusToUse}/{maxCpus}
							<button type="button" class="common-btn-sm-filled w-fit" on:click={() => cpusToUse = maxCpus}>Max</button>

							<input
							type="range"
								bind:value={cpusToUse}
								max={maxCpus}
								min=1
							/>
							</label>
							
							<button type="button" class="common-btn-sm-filled w-fit" on:click={() => { mine(prefixList, cpusToUse)}}>
								<span><MiningIcon size={16} /></span>
								<span>{mined_id.length ? 'Mine NEW' : 'Mine it'}</span>
							</button>
					
							{#if mined_id.length}
							<section class="card p-2">
								<p>Mined npub:</p>
								<code>{truncateString(mined_id[0], 16)}</code>
							</section>	
							{/if}
						{/if}
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
