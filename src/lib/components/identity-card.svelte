<script lang="ts">
	import { decrypt, deleteIdentityFromDb, encodeSeedToNsec, generateQRCode, truncateString } from "$lib/resources/helpers";
	import PlusIcon from "$lib/resources/icons/plus-icon.svelte";
	import { appContextStore } from "$lib/stores/stores";
	import type { ProfileInterface } from "$lib/types/interfaces";
	import { Accordion, AccordionItem, Avatar } from "@skeletonlabs/skeleton";
	import { nip19 } from "nostr-tools";
	import { npubEncode } from "nostr-tools/nip19";
	import { get } from "svelte/store";

    export let profile: ProfileInterface
    let secret: string | undefined
    let qrCode: string | undefined
    async function handleDelete() {
        const appContext = get(appContextStore);
        if (!appContext?.currentDbname) return
        deleteIdentityFromDb(appContext?.currentDbname, 'hexpub', profile.hexpub);
    }

    async function revealSecret(value: string): Promise<string> {
        secret = await decrypt(value, $appContextStore?.sessionPass!)
        qrCode = generateQRCode(secret!)
        console.log(qrCode)
        return secret!
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
                <section class=" break-words ">
                {#if profile?.xpub != "NULL"}
                    <code>Xpub: {profile.xpub}</code>
                {/if}
                {#if profile?.gap != 0}
                <p>Gap: {profile.gap}</p>
                {/if}
                <p>Public Key: {npubEncode(profile.hexpub)}</p>
                <p>Private Key: {secret ? encodeSeedToNsec(secret) : 'Private key is encrypted'}</p>
                {#if qrCode}
                    <Avatar
                    border="border-4 border-surface-300-600-token hover:!border-primary-500"
                    cursor="cursor-pointer"
                    src={qrCode} 
                    width=" w-36"
                    rounded="rounded-3xl"
                    alt="QR Code"
                    />
                {/if}
                <button class="btn btn-sm variant-filled" on:click={() => revealSecret(profile?.prvk ? profile?.prvk : '')}>Reveal</button>
                </section>
                <hr/>
            </svelte:fragment>
        </AccordionItem>
    </Accordion>
</section>
</div>