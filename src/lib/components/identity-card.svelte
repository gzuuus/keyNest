<script lang="ts">
	import { deleteIdentityFromDb, truncateString } from "$lib/resources/helpers";
	import { appContextStore } from "$lib/stores/stores";
	import type { ProfileInterface } from "$lib/types/interfaces";
	import { Accordion, AccordionItem } from "@skeletonlabs/skeleton";
	import { nip19 } from "nostr-tools";
	import { get } from "svelte/store";

    export let profile: ProfileInterface
    async function handleDelete() {
        const appContext = get(appContextStore);
        if (!appContext?.currentDbname) return
        deleteIdentityFromDb(appContext?.currentDbname, 'hexpub', profile.hexpub);
    }
</script>

<div class=" card flex flex-col gap-1 px-4 py-2">
<h3 class="h4">{profile.name}</h3>
<p>{truncateString(nip19.npubEncode(profile.hexpub))}</p>

<button class="common-btn-sm-ghost-error w-fit" on:click={handleDelete}>Delete</button>
<section class=" max-w-full">
    <Accordion>
        <AccordionItem>
            <svelte:fragment slot="summary">More info</svelte:fragment>
            <svelte:fragment slot="content">
                <code>Xpub: {profile.xpub}</code>
                <p>Gap: {profile.gap}</p>
                <code class="text-sm whitespace-pre-wrap">{JSON.stringify(profile)}</code>
                <hr/>
            </svelte:fragment>
        </AccordionItem>
    </Accordion>
</section>
</div>