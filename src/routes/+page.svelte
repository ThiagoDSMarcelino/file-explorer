<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import StorageCard from "$lib/StorageCard.svelte";

    let output: StorageModel[] = [];

    onMount(async () => {
        output = await invoke("get_all_drivers");
    });
</script>

{#if output}
    Found storages:
    <div>
        {#each output as storage}
            <StorageCard storageData={storage} />
        {/each}
    </div>
{/if}
