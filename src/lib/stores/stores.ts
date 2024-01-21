import type { RootPInterface } from '$lib/types/profile-json-interface';
import { type Writable, writable } from 'svelte/store';

export const fileStore: Writable<String[] | null> = writable([]);
export const currentProfile: Writable<RootPInterface | undefined> = writable(undefined);
