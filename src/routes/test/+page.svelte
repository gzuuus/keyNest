<script lang="ts">
	import { calculateXprvFromSeed, decrypt, derive_child_xprv_from_xprv } from "$lib/resources/helpers";
	import { currentProfile } from "$lib/stores/stores";
	import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api";

  async function test() {
    let executeDb = await invoke("update_identity_in_db", { 
      dbName: "hello.db",
      column: "npub",
      value: "bd52af3b5d4939bdc097178fd6e2194db9d55b8f76775ad050fc8989f6894ae8",
      newValue: "npubTest"
    });
    console.log(executeDb);
  }
  // let result = update_identity_in_db("my_db", "name", "John", "Jane");

</script>
<button on:click={test}>Try test</button>