<script lang="ts">
	import { deleteIdentityFromDb, truncateString } from "$lib/resources/helpers";
	import PlusIcon from "$lib/resources/icons/plus-icon.svelte";
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
<div class=" flex flex-col gap-1">
<h1 class="h1">{profile.name}</h1>
<p class="badge w-fit variant-soft">{truncateString(nip19.npubEncode(profile.hexpub))}</p>

<section class="w-fit max-w-full">
<Accordion regionControl=" variant-soft" >
    <AccordionItem>
        <svelte:fragment slot="lead"><PlusIcon size={16}/></svelte:fragment>
        <svelte:fragment slot="summary">More info</svelte:fragment>
        <svelte:fragment slot="content">
            <code>Xpub: {profile.xpub}</code>
            <p>Gap: {profile.gap}</p>
            <code class="text-sm">{JSON.stringify(profile)}</code>
            <hr/>
        </svelte:fragment>
    </AccordionItem>
</Accordion>
</section>
</div>