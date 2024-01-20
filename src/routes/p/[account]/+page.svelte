<script lang="ts">
	import { decrypt, derive_child_pub_from_xpub } from '$lib/resources/helpers';
	import { currentProfile } from '$lib/stores/stores';
</script>

{#if $currentProfile}
	<div class="card p-4">
		<h1 class="h2">{$currentProfile?.name}</h1>
		<section class=" max-w-sm">
			<pre class=" whitespace-pre-wrap break-words">{JSON.stringify($currentProfile, null, 2)}</pre>
		</section>
		<button class="common-btn-sm-filled" on:click={() => decrypt($currentProfile?.prvk, '123')}
			>Decrypt</button
		>
		<button
			class="common-btn-sm-filled"
			on:click={() => derive_child_pub_from_xpub($currentProfile?.xpub ?? '', 2)}>Derive</button
		>
	</div>
{/if}
