<script lang="ts">
	import { mine_id, num_cpus } from "$lib/resources/helpers";
	import { LightSwitch } from "@skeletonlabs/skeleton";
	import { invoke } from "@tauri-apps/api";
	async function handleClick() {
		let path = await invoke("test_dir");
		console.log(path);
	}

	async function mine(prefixes: string[]): Promise<string[]> {
		let mined_id: string[] = await mine_id(prefixes, 2);
		console.log(mined_id)
		return mined_id
	}

	async function count_cpus(): Promise<number> {
		let cpus:number = await num_cpus();
		console.log(cpus)
		return cpus
	}
</script>
<p>Light/Dark mode:</p>
<LightSwitch />
<button class="btn variant-filled" on:click={handleClick}>Test</button>
<button class="btn variant-filled" on:click={() => mine(["gz"])}>Mine</button>
<button class="btn variant-filled" on:click={() => count_cpus()}>CPUs</button>