<script lang="ts">
	import { mine_id, num_cpus } from "$lib/resources/helpers";
	import { LightSwitch } from "@skeletonlabs/skeleton";
	import { invoke } from "@tauri-apps/api";
	import { onMount } from "svelte";

	let path:string;

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
	// TODO: Clean this up, 
	// feature: Change folder, open folder
	// choose theme
	// choose language
	// choose security level
	onMount(async () => {
		path = await invoke("test_dir");
	})
</script>
<div class=" flex flex-col gap-2">
	<p class="font-bold">Light/Dark mode:</p>
	<LightSwitch />
	<p class="font-bold">Path for dbs:</p>
	<p>{path}</p>
</div>