import { goto } from '$app/navigation';
import { currentProfile, fileStore } from '$lib/stores/stores';
import type { RootPInterface } from '$lib/types/profile-json-interface';
import { invoke } from '@tauri-apps/api';

export async function writeFile(name: string, data: any): Promise<boolean> {
	return await invoke('write_json', { name, data });
}

export async function read(name: string): Promise<RootPInterface> {
	let content: RootPInterface = await invoke('read_file', { name });
	console.log(content);
	currentProfile.set(content);
	return content;
}

export async function deleteFile(fileName: string) {
	try {
		let deleteFile = await invoke('delete_file_by_name', { filename: fileName });
		if (deleteFile) listFiles();
	} catch (error) {
		console.log(error);
	}
}

export async function listFiles(): Promise<string[] | undefined> {
	let fileList: string[] = await invoke('list_files');
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

export async function encrypt(to_encrypt: string, key: string): Promise<string> {
	let encrypted: string = await invoke('encrypt_string', { toEncrypt: to_encrypt, key: key });
	return encrypted;
}

export async function decrypt(
	to_decrypt: string | undefined,
	key: string
): Promise<string | undefined> {
	if (!to_decrypt) return undefined;
	let decrypted: string = await invoke('decrypt_cypher', { toDecrypt: to_decrypt, key: key });
	console.log(decrypted);
	return decrypted;
}

export async function derive_child_pub_from_xpub(
	xpub: string,
	child_index: number
): Promise<string> {
	let derived: string = await invoke('derive_child_pub_from_xpub', {
		xpub: xpub,
		childIndex: child_index
	});
	derived = derived.substring(2);
	console.log(derived);
	return derived;
}

export async function derive_child_xprv_from_xprv(
	xprv: string,
	child_index: number
): Promise<string> {
	let derived: string = await invoke('derive_child_xprv_from_xprv', {
		xprv: xprv,
		childIndex: child_index
	});
	console.log(derived);
	return derived;
}

export async function calculateXprvFromSeed(seed: string): Promise<string> {
	let xprv: string = await invoke('calculate_xprv_from_seed', { seed: seed });
	return xprv;
}

export async function calculateXpubFromSeed(seed: string): Promise<string> {
	let xpub: string = await invoke('calculate_xpub_from_seed', { seed: seed });
	console.log(xpub);
	return xpub;
}

export function uint8ArrayTo32HexString(uint8Array: Uint8Array): string {
	return [...uint8Array]
		.map((b) => b.toString(16).padStart(2, '0'))
		.slice(0, 32)
		.join('');
}
