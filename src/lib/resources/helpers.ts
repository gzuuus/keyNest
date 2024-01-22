import { goto } from '$app/navigation';
import { currentProfile, fileStore } from '$lib/stores/stores';
import type { ProfileInterface} from '$lib/types/profile-json-interface';
import { invoke } from '@tauri-apps/api';

export async function writeFile(name: string, data: any): Promise<boolean> {
	return await invoke('write_json', { name, data });
}

export async function insertInDb(name: string, data: any): Promise<boolean> {
	let dataValues: ProfileInterface = data;
	return await invoke('insert_into_db', { 
		dbName: name, 
		name: dataValues.name,
		npub: dataValues.npub,
		xpub: dataValues.xpub,
		prvk: dataValues.prvk,
		level: dataValues.level?.toString(),
		gap: dataValues.gap?.toString(),
		parent: dataValues.parent 
	  });
}

export async function read(name: string): Promise<ProfileInterface> {
	let content: ProfileInterface = await invoke('read_file', { name });
	currentProfile.set(content);
	return content;
}

export async function getRootbyColumnAndValue(dbName: string, column: string, value: string): Promise<ProfileInterface[]> {
    let content: ProfileInterface[] = await invoke("get_root_identity_by_column_and_value", { 
      dbName: dbName,
      column: column,
      value: value
    });
    currentProfile.set(content[0]);
	return content;
  }

export async function readDb(dbName: string) {
    let content: ProfileInterface[] = await invoke("get_all_identities", { 
      dbName: dbName,
    });
  }

export async function deleteFile(fileName: string): Promise<boolean> {
	try {
		let deleteFile = await invoke('delete_file_by_name', { filename: fileName });
		if (deleteFile) listFiles();
		return true;
	} catch (error) {
		console.log(error);
		return false;
	}
}

// TODO: Fix this, if there are two list and you delete one it doesnt update
export async function listFiles(): Promise<string[] | undefined> {
	let fileList: string[] | undefined = await invoke('list_files');
	console.log(fileList);
	!fileList?.length ? fileList = undefined : fileList
	if (fileList) {
		goto('/');
		fileStore.set(fileList);
		return fileList;
	} else if (fileList! == undefined) {
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
	return derived;
}
//insert_into_db(db_name: &str, name: &str, npub: &str, xpub: &str, prvk: &str, level: &str, gap: &str, parent: &str)
// TODO!
export async function insertDerivedChild(
	db_name: string,
	pubk: string,
): Promise<boolean> {
	try {
		let derived: boolean = await invoke('insert_into_db', {
			dbName: db_name,
			name: 'derived',
			npub: pubk,
			xpub: '',
			prvk: '',
			level: '1',
			gap: '0',
			parent: ''
		});
		return true;
	} catch (error) {
		console.log(error);
		return false;
	}
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

export function logOut() {
	currentProfile.set(undefined);
	goto('/')
}
