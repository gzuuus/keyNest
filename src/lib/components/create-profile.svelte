<script lang="ts">
	import AddFileIcon from '$lib/resources/icons/add-file-icon.svelte';
	import type { RootPInterface } from '$lib/types/profile-json-interface';
	import { FileDropzone, focusTrap } from '@skeletonlabs/skeleton';
	import { getToastStore } from '@skeletonlabs/skeleton';
	import { goto } from '$app/navigation';
	import { object, string } from 'zod';
	import { generateSecretKey, getPublicKey, nip19 } from 'nostr-tools';
	import CloseIcon from '$lib/resources/icons/close-icon.svelte';
	import { calculateXpubFromSeed, encrypt, writeFile } from '$lib/resources/helpers';

	export const rootPSchema = object({
		name: string(),
		npub: string()
	});

	interface CreateAccForm {
		name: string;
		pass: string;
	}

	let formData: Partial<CreateAccForm> = {};

	let name: string;
	let pass: string;

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
								const data: RootPInterface = JSON.parse(fileContent);
								console.log(data);
								const writeFiles = await writeFile(data.name, data);
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
			return !!formData && Object.values(formData).every(Boolean);
		}
		function readonlyValue(value: unknown): unknown {
			return typeof value === 'object' ? { ...value } : value;
		}
		formData = {
			name,
			pass
		};
		function uint8ArrayTo32HexString(uint8Array: Uint8Array): string {
			return [...uint8Array]
				.map((b) => b.toString(16).padStart(2, '0'))
				.slice(0, 32)
				.join('');
		}

		let insecurePrvk = generateSecretKey();
		let npub = nip19.npubEncode(getPublicKey(insecurePrvk));
		let encryptedNsec = await encrypt(uint8ArrayTo32HexString(insecurePrvk), pass);
		let xpub = await calculateXpubFromSeed(uint8ArrayTo32HexString(insecurePrvk));
		let extendedFormData = {
			name,
			npub: npub,
			xpub: xpub,
			prvk: encryptedNsec,
			level: 0,
			scope: 0
		};

		if (!validateFormData()) return;
		try {
			const formValues = Object.fromEntries(
				Object.entries(extendedFormData).map(([key, val]) => [key, readonlyValue(val)])
			) as Readonly<Record<keyof RootPInterface, unknown>>;
			console.log(formValues);
			let craftFile = await writeFile(formData.name!, formValues);
			console.log(craftFile);
			createNew = false;
			goto('/');
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
		<form use:focusTrap={true} on:submit|preventDefault={handleSubmit}>
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
			<button class="common-btn-sm-filled" type="submit">Publish</button>
		</form>
	</section>
{/if}
