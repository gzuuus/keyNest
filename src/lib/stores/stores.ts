import type { AppContext, ProfileInterface } from '$lib/types/interfaces';
import { type Writable, writable } from 'svelte/store';


export const appContextStore: Writable<AppContext | undefined> = writable({fileList: undefined, currentDbname: undefined});
export const currentProfile: Writable<ProfileInterface | undefined> = writable(undefined);
export const derivedIdentitiesStore: Writable<ProfileInterface[] | undefined> = writable(undefined)