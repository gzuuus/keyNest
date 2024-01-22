import type { ProfileInterface } from '$lib/types/profile-json-interface';
import { type Writable, writable } from 'svelte/store';

export const fileStore: Writable<String[] | undefined> = writable([]);
export const currentProfile: Writable<ProfileInterface | undefined> = writable(undefined);
