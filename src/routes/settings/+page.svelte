<script lang="ts">
	import { LightSwitch } from "@skeletonlabs/skeleton";
	import { invoke } from "@tauri-apps/api";
	async function handleClick() {
		let path = await invoke("test_dir");
		console.log(path);
	}
</script>
<p>Light/Dark mode:</p>
<LightSwitch />
<button class="btn variant-filled" on:click={handleClick}>Test</button>