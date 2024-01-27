<script lang="ts">
	import { decrypt, deleteIdentityFromDb, hexStringToUint8Array, mnemonics_from_seed, truncateString } from "$lib/resources/helpers";
	import PlusIcon from "$lib/resources/icons/plus-icon.svelte";
	import { appContextStore } from "$lib/stores/stores";
	import type { ProfileInterface } from "$lib/types/interfaces";
	import { Accordion, AccordionItem, Avatar } from "@skeletonlabs/skeleton";
	import { nip19 } from "nostr-tools";
	import { get } from "svelte/store";
    import QRcode from "qrcode-generator";
	import QrIcon from "$lib/resources/icons/qr-icon.svelte";
	import KeyIcon from "$lib/resources/icons/key-icon.svelte";

    export let profile: ProfileInterface
    let qrImageUrl: string = "";
    let showQR: boolean = false;
    let showSecretQr: boolean = false;
    let secret: string | undefined
    let mnemonics: string | undefined
    function generateQRCode(value: string) {
        let qr = QRcode(0, "L");
        qr.addData(value);
        qr.make();
        showQR = !showQR;
        qrImageUrl = qr.createDataURL();
        return qrImageUrl;
    }
    async function revealSecret(value: string): Promise<string> {
        secret = await decrypt(value, $appContextStore?.sessionPass!)
        mnemonics = await mnemonics_from_seed(secret!)
        console.log(mnemonics)
        return secret!
    }

    function encodeSeedToNsec ( value: string): string {
        let uint8array = hexStringToUint8Array(value);
        let nsec = nip19.nsecEncode(uint8array);
        return nsec
    }
    async function generateSecretQRCode(value: string) {
        secret = await revealSecret(value)
        value = secret ? secret : ''
        let qr = QRcode(0, "L");
        qr.addData(value);
        qr.make();
        showSecretQr = !showSecretQr;
        qrImageUrl = qr.createDataURL();
        if (!showSecretQr) qrImageUrl = "";
        return qrImageUrl;
    }
</script>
<div class=" flex flex-row justify-between gap-2">
<section class=" flex flex-col gap-2">
    <h1 class="h1">{profile.name}</h1>
    <section class=" inline-flex gap-1">
        <p class="badge w-fit variant-soft ">{truncateString(nip19.npubEncode(profile.hexpub))}</p>
        <button class=" badge {showQR ? 'variant-filled' : 'variant-soft'}"  on:click={() => generateQRCode(nip19.npubEncode(profile.hexpub))}
            ><QrIcon size={16} /></button
        >
        <button class=" badge {showSecretQr ? 'variant-filled' : 'variant-soft'}"  on:click={() => generateSecretQRCode(profile?.prvk ? profile?.prvk : '') }
            ><KeyIcon size={16} /></button
        >
    </section>
    <section class="w-fit max-w-screen-sm">
    <Accordion regionControl=" variant-soft" >
        <AccordionItem>
            <svelte:fragment slot="lead"><PlusIcon size={16}/></svelte:fragment>
            <svelte:fragment slot="summary">More info</svelte:fragment>
            <svelte:fragment slot="content">
                <code>Xpub: {profile.xpub}</code>
                <p>Gap: {profile.gap}</p>
                <p>Private Key: {secret ? encodeSeedToNsec(secret) : profile.prvk}</p>
                {#if mnemonics}
                <p>Mnemonics: {mnemonics}</p>
                {/if}
                <button class="btn btn-sm variant-filled" on:click={() => revealSecret(profile?.prvk ? profile?.prvk : '')}>Reveal</button>
                <hr/>
                <code class="text-sm">{JSON.stringify(profile)}</code>
                <hr/>
            </svelte:fragment>
        </AccordionItem>
    </Accordion>
    </section>
</section>
<section >
    <Avatar class="{showQR || showSecretQr ? 'common-ring' : 'hidden'}" 
    border="border-4 border-surface-300-600-token hover:!border-primary-500"
    cursor="cursor-pointer"
    src={qrImageUrl} 
    width="w-28"
    rounded="rounded-3xl"
    alt="QR Code"
    />
</section>
</div>