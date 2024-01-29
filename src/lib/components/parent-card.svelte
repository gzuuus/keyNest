<script lang="ts">
	import { decrypt, encodeSeedToNsec, generateQRCode, truncateString } from "$lib/resources/helpers";
	import PlusIcon from "$lib/resources/icons/plus-icon.svelte";
	import { appContextStore } from "$lib/stores/stores";
	import type { ProfileInterface } from "$lib/types/interfaces";
	import { Accordion, AccordionItem, Avatar } from "@skeletonlabs/skeleton";
	import { nip19 } from "nostr-tools";
	import QrIcon from "$lib/resources/icons/qr-icon.svelte";
	import { npubEncode } from "nostr-tools/nip19";

    export let profile: ProfileInterface
    let publicQrCode: string | undefined
    let showQR: boolean = false
    let secretQrCode: string | undefined
    let secret: string | undefined

    async function revealSecret(value: string): Promise<string> {
        secret = await decrypt(value, $appContextStore?.sessionPass!)
        secretQrCode = generateQRCode(secret!)
        return secret!
    }

    function handleShowPubQr() {
        showQR = !showQR
        if (!publicQrCode) {
            publicQrCode = generateQRCode(nip19.npubEncode(profile.hexpub))
        }
    }
</script>
<div class=" flex flex-row justify-between gap-2">
<section class=" flex flex-col gap-2">
    <h1 class="h1">{profile.name}</h1>
    <section class=" inline-flex gap-1">
        <p class="badge w-fit variant-soft ">{truncateString(nip19.npubEncode(profile.hexpub))}</p>
        <button class=" badge {showQR ? 'variant-filled' : 'variant-soft'}"  on:click={() => handleShowPubQr() }
            ><QrIcon size={16} /></button
        >
    </section>
    <section class="w-fit max-w-screen-sm">
    <Accordion regionControl=" variant-soft" >
        <AccordionItem>
            <svelte:fragment slot="lead"><PlusIcon size={16}/></svelte:fragment>
            <svelte:fragment slot="summary">More info</svelte:fragment>
            <svelte:fragment slot="content">
                <code>Xpub: {profile.xpub}</code>
                {#if profile.gap != 0}
                <p>Gap: {profile.gap}</p>
                {/if}
                <p>Public Key: {npubEncode(profile.hexpub)}</p>
                <p>Private Key: {secret ? encodeSeedToNsec(secret) : 'Private key is encrypted'}</p>
                {#if secretQrCode}
                    <Avatar
                    border="border-4 border-surface-300-600-token hover:!border-primary-500"
                    cursor="cursor-pointer"
                    src={secretQrCode} 
                    width=" w-36"
                    rounded="rounded-3xl"
                    alt="QR Code"
                    />
                {/if}
                <button class="btn btn-sm variant-filled" on:click={() => revealSecret(profile?.prvk ? profile?.prvk : '')}>Reveal</button>
                <hr/>
            </svelte:fragment>
        </AccordionItem>
    </Accordion>
    </section>
</section>
<section >
    {#if publicQrCode && showQR}
    <Avatar
    border="border-4 border-surface-300-600-token hover:!border-primary-500"
    cursor="cursor-pointer"
    src={publicQrCode} 
    width="w-28"
    rounded="rounded-3xl"
    alt="QR Code"
    />
    {/if}
</section>
</div>