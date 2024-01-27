<script lang="ts">
	import { deleteIdentityFromDb, truncateString } from "$lib/resources/helpers";
	import PlusIcon from "$lib/resources/icons/plus-icon.svelte";
	import { appContextStore } from "$lib/stores/stores";
	import type { ProfileInterface } from "$lib/types/interfaces";
	import { Accordion, AccordionItem } from "@skeletonlabs/skeleton";
	import { nip19 } from "nostr-tools";
	import { get } from "svelte/store";

    export let profile: ProfileInterface
    console.log(profile.child_index)
    async function handleDelete() {
        const appContext = get(appContextStore);
        if (!appContext?.currentDbname) return
        deleteIdentityFromDb(appContext?.currentDbname, 'hexpub', profile.hexpub);
    }

</script>

<div class=" card flex flex-col gap-1 px-4 py-2">
<h3 class="h4">{profile.name}:{profile.child_index}</h3>
<p class="badge w-fit variant-soft ">{truncateString(nip19.npubEncode(profile.hexpub))}</p>

<button class="common-btn-sm-ghost-error w-fit" on:click={handleDelete}>Delete</button>
<section class=" max-w-full">
    <Accordion>
        <AccordionItem>
            <svelte:fragment slot="lead"><PlusIcon size={12}/></svelte:fragment>
            <svelte:fragment slot="summary">More info</svelte:fragment>
            <svelte:fragment slot="content">
                <code>Xpub: {profile.xpub}</code>
                <p>Gap: {profile.gap}</p>
                <section class=" max-w-prose">
                    <code class="text-sm">{JSON.stringify(profile, null, 2)}</code>
                </section>
                <hr/>
            </svelte:fragment>
        </AccordionItem>
    </Accordion>
</section>
</div>