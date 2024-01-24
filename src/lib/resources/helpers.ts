import { goto } from '$app/navigation';
import { appContextStore, currentProfile, derivedIdentitiesStore } from '$lib/stores/stores';
import type { ProfileInterface} from '$lib/types/interfaces';
import { invoke } from '@tauri-apps/api';

export async function writeFile(name: string, data: any): Promise<boolean> {
	return await invoke('write_json', { name, data });
}

export async function insertInDb(name: string, data: ProfileInterface): Promise<boolean> {
	return await invoke('insert_into_db', { 
		dbName: `${name}.db`, 
		name: data.name,
		npub: data.npub,
		xpub: data.xpub,
		prvk: data.prvk,
		level: data.level?.toString(),
		gap: data.gap?.toString(),
		parent: data.parent 
	  });
}

// export async function read(dbName: string): Promise<ProfileInterface> {
// 	let content: ProfileInterface = await invoke('read_file', { dbName: dbName });
// 	console.log('This is the content', content);
// 	currentProfile.set(content);
// 	return content;
// }

export async function getRootbyColumnAndValue(dbName: string, column: string, value: string): Promise<ProfileInterface[]> {
    let content: ProfileInterface[] = await invoke("get_root_identity_by_column_and_value", { 
      dbName: dbName,
      column: column,
      value: value
    });
    currentProfile.set(content[0]);
	appContextStore.update((value) => {
		return { 
			fileList: value?.fileList, 
			currentDbname: dbName };
	})
	return content;
  }

export async function readDb(dbName: string): Promise<ProfileInterface[]> {
	console.log('readDb', dbName);
    let content: ProfileInterface[] = await invoke("get_all_identities", { 
      dbName: dbName,
    });
	console.log(content);
	derivedIdentitiesStore.set(content);
	return content;
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

// TODO Implement this
export async function updateValueInDb(dbName: string, column: string, value: string, newValue: string): Promise<boolean> {
    try {
		let updateDb: boolean = await invoke("update_identity_in_db", { 
			dbName: dbName,
			column: column,
			value: value,
			newValue: newValue
		  });
		  if (updateDb) readDb(dbName)
		  return updateDb;
	} catch (error) {
		console.log(error);
		return false;
	}
  }

export async function deleteIdentityFromDb(dbName: string, column: string, value: string): Promise<boolean> {
	console.log(dbName, column, value);
	try {
		let deleteFile: boolean = await invoke('delete_identity_from_db', { 
			dbName: dbName,
			column: column,
			value: value
		  });
		if (deleteFile) readDb(dbName)
		return deleteFile;
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
		appContextStore.set({fileList: fileList, currentDbname: undefined});
		return fileList;
	} else if (fileList! == undefined) {
		console.log('no files');
		goto('/create-profile');
		appContextStore.set({fileList: undefined, currentDbname: undefined});
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
	appContextStore.set({
		fileList: undefined, 
		currentDbname: undefined
	});
	goto('/')
}
