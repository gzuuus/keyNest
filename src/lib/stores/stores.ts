import type { ProfileJsonInterface } from "$lib/types/profile-json-interface";
import { type Writable, writable } from "svelte/store";

export const fileStore: Writable<String[]> = writable([]);
export const currentProfile: Writable<ProfileJsonInterface | undefined> = writable(undefined);