<script lang="ts">
	import { deleteIdentityFromDb } from "$lib/resources/helpers";
	import { appContextStore } from "$lib/stores/stores";
	import type { ProfileInterface } from "$lib/types/interfaces";
	import { nip19 } from "nostr-tools";
	import { get } from "svelte/store";

    export let profile: ProfileInterface
    async function handleDelete() {
        const appContext = get(appContextStore);
        if (!appContext?.currentDbname) return
        deleteIdentityFromDb(appContext?.currentDbname, 'hexpub', profile.hexpub);
    }
</script>

<p>{profile.name}</p>
<p>{nip19.npubEncode(profile.hexpub)}</p>
<button class="common-btn-sm-ghost-error" on:click={handleDelete}>Delete</button> 
<code>{JSON.stringify(profile)}</code>