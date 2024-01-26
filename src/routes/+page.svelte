<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import DriveCard from "../components/DriveCard.svelte";

    let output: DriverModel[] = [];

    onMount(async () => {
        output = await invoke("get_all_drivers");
    });
</script>

{#if output}
    <h2>Drivers Founded:</h2>
    <div>
        {#each output as storage}
            <DriveCard storageData={storage} />
        {/each}
    </div>
{/if}

<style>
    h2 {
        text-align: center;
    }
</style>
