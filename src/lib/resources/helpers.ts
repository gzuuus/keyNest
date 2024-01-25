import { goto } from '$app/navigation';
import { appContextStore, currentProfile, derivedIdentitiesStore } from '$lib/stores/stores';
import type { ProfileInterface} from '$lib/types/interfaces';
import { invoke } from '@tauri-apps/api';
import { getPublicKey } from 'nostr-tools';

export async function writeFile(name: string, data: any): Promise<boolean> {
	return await invoke('write_json', { name, data });
}

export async function insertInDb(name: string, data: ProfileInterface): Promise<boolean> {
	return await invoke('insert_into_db', { 
		dbName: name, 
		name: data.name,
		hexpub: data.hexpub,
		xpub: data.xpub,
		prvk: data.prvk,
		level: data.level?.toString(),
		gap: data.gap?.toString(),
		parent: data.parent,
		childIndex: data.childIndex?.toString()
	  });
}

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
			currentDbname: dbName,
			sessionPass: value?.sessionPass
		}
	})
	return content;
  }

export async function readDb(dbName: string): Promise<ProfileInterface[]> {
	// console.log('readDb', dbName);
    let content: ProfileInterface[] = await invoke("get_all_identities", { 
      dbName: dbName,
    });
	// console.log(content);
	derivedIdentitiesStore.set(content);
	return content;
  }

// TODO dont delete, just mark as deleted
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
export async function updateValueInDb(dbName: string, column: string, newValue: string, whereColumn: string, value: string): Promise<boolean> {
    try {
		let updateDb: boolean = await invoke("update_identity_in_db", { 
			dbName: dbName,
			column: column,
			value: value,
			whereColumn: whereColumn,
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

export async function listFiles(): Promise<string[] | undefined> {
	console.log('listFiles');
	let fileList: string[] | undefined = await invoke('list_files');
	!fileList?.length ? fileList = undefined : fileList
	if (fileList) {
		goto('/');
		appContextStore.update((value) => {
			return { 
				fileList: fileList, 
				currentDbname: value?.currentDbname,
				sessionPass: value?.sessionPass
			}
		})
		return fileList;
	} else if (fileList! == undefined) {
		console.log('no files');
		goto('/create-profile');
		// appContextStore.set({fileList: undefined, currentDbname: undefined});
		appContextStore.update((value) => {
			return { 
				fileList: undefined, 
				currentDbname: undefined,
				sessionPass: undefined
			}
		})
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

export async function derive_child_pub_from_xpub_and_insert(
	parentIdentity: ProfileInterface,
	dbName: string,
): Promise<string | false> {
	let derivedChildPub: string = await invoke('derive_child_pub_from_xpub', {
		xpub: parentIdentity.xpub,
		childIndex: parentIdentity.gap
	});

	if (!derivedChildPub) return false;
	let parentLevel = parentIdentity.level;
	const derivedProfile: ProfileInterface = {
		name: 'derived',
		hexpub: derivedChildPub,
		xpub: 'NULL',
		prvk: 'NULL',
		level: ++parentLevel!,
		gap: 0,
		parent: parentIdentity.hexpub,
		childIndex: parentIdentity.gap
	}

	const isInserted = await insertDerivedChild(dbName, derivedProfile);
	const updateParent = !isInserted ? false : await updateValueInDb(
		dbName,
		'gap',
		(++parentIdentity.gap!).toString(),
		'hexpub',	
		parentIdentity.hexpub,
	)
	if (!updateParent) return false;

	return derivedChildPub.substring(2);
}

export function hexStringToUint8Array(hexString: string): Uint8Array {
	if (hexString.length % 2 !== 0) {
	  throw new Error("Invalid hex string. Length should be even.");
	}
  
	const numberOfElements = hexString.length / 2;
	return new Uint8Array(numberOfElements).map((_, index) => {
	  const start = index * 2;
	  const end = start + 2;
	  const hexSubstring = hexString.substring(start, end);
	  return parseInt(hexSubstring, 16);
	});
  }

export async function derive_child_from_seed_and_insert(
	parentIdentity: ProfileInterface,
	password: string,
	dbName: string,
): Promise<string | false> {
	const seed = await decrypt(parentIdentity.prvk, password)
	const xprv = seed ? await calculateXprvFromSeed(seed) : '';
	const childSeed = await derive_child_seed_from_xpriv(xprv!, parentIdentity.gap!);
	const encryptedChildSeed = childSeed ? await encrypt(childSeed, password) : '';
	const childHexPub = childSeed ? getPublicKey(hexStringToUint8Array(childSeed)) : '';
	let parentLevel:number = parentIdentity.level!;

	const derivedProfile: ProfileInterface = {
		name: 'derived',
		hexpub: childHexPub,
		xpub: 'NULL',
		prvk: encryptedChildSeed ? encryptedChildSeed : 'NULL',
		level: ++parentLevel!,
		gap: 0,
		parent: parentIdentity.hexpub,
		childIndex: parentIdentity.gap
	}
	const isInserted = await insertDerivedChild(dbName, derivedProfile);
	const updateParent = !isInserted ? false : await updateValueInDb(
		dbName,
		'gap',
		(++parentIdentity.gap!).toString(),
		'hexpub',	
		parentIdentity.hexpub,
	)
	if (!updateParent) return false;

	return childHexPub.substring(2);
}

export async function derive_child_pub_from_xpub(
	parentIdentity: ProfileInterface,
	child_index: number
): Promise<string | false> {
	let derivedChildPub: string = await invoke('derive_child_pub_from_xpub', {
		xpub: parentIdentity.xpub,
		childIndex: child_index
	});

	return derivedChildPub.substring(2);
}

export async function derive_child_seed_from_xpriv(
	xprv: string,
	child_index: number,
): Promise<string | false> {
	// xprv = await derive_child_xprv_from_xprv
	console.log(xprv);
	let derivedChildSeed: string = await invoke('derive_child_seed_from_xprv', {
		xprv: xprv,
		childIndex: child_index
	});

	return derivedChildSeed;
}

// TODO!
export async function insertDerivedChild(
	db_name: string,
	pData: ProfileInterface,
): Promise<boolean> {
	try {
		console.log('PDATA',pData);
		let derived: boolean = await invoke('insert_into_db', {
			dbName: db_name,
			name: pData.name,
			hexpub: pData.hexpub,
			xpub: pData.xpub,
			prvk: pData.prvk,
			level: pData.level?.toString(),
			gap: pData.gap?.toString(),
			parent: pData.parent,
			childIndex: pData.childIndex?.toString()
		});
		return derived;
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

export function truncateString(str: string): string {
	  return str.substring(0, 12) + ":" + str.substring(str.length - 6);
}

export function logOut() {
	currentProfile.set(undefined);
	appContextStore.set({
		fileList: undefined, 
		currentDbname: undefined,
		sessionPass: undefined
	});
	goto('/')
}
