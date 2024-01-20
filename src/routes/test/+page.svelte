<script lang="ts">
	import { calculateXprvFromSeed, decrypt, derive_child_xprv_from_xprv } from "$lib/resources/helpers";
	import { currentProfile } from "$lib/stores/stores";
	import { onMount } from "svelte";

  onMount(async () => {
    if (!$currentProfile) return
    let seed = await decrypt($currentProfile?.prvk, '123')
    let xprv = await calculateXprvFromSeed(seed!)
    let childXprv = await derive_child_xprv_from_xprv(xprv, 30)
    console.log(seed, xprv, childXprv)
  })
</script>
