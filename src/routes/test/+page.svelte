<script lang="ts">
	import { calculateXprvFromSeed, decrypt, derive_child_xprv_from_xprv } from "$lib/resources/helpers";
	import { currentProfile } from "$lib/stores/stores";
	import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";

  async function test() {
    let executeDb = await invoke("root_name", { 
      dbName: "hello",
    });
    console.log(executeDb);
  }
</script>
<button on:click={test}>Try test</button>