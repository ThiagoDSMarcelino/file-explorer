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
    <div class="container">
        {#each output as storage}
            <DriveCard driver={storage} />
        {/each}
    </div>
{/if}

<style>
    h2 {
        text-align: center;
    }

    .container {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
        row-gap: 16px;
        column-gap: 16px;
    }
</style>
