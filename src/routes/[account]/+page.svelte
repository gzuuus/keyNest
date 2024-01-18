<script lang="ts">
	import { page } from '$app/stores';
	import { currentProfile } from '$lib/stores/stores';
	$: console.log($page.params.account);
</script>

<div class="card p-4">
	<h1 class="h2">{$currentProfile?.name}</h1>
	<h2>{$currentProfile?.npub}</h2>
</div>
