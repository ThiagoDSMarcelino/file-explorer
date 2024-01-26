<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import ProgressBar from "./ProgressBar.svelte";

    export let driver: DriverModel;

    const imagePath = driver.kind.toLocaleLowerCase() + ".png";

    const cardClicked = async () => {
        const res = await invoke("get_data_from_dir", {
            path: driver.dir,
        });

        console.log(res);
    };
</script>

<button class="card" on:click={cardClicked}>
    <div class="card-content">
        <img src={imagePath} alt="Storage" />
        <div class="info">
            <h3>{driver.name} - {driver.dir}</h3>
            <p><strong>Driver Kind:</strong> {driver.kind}</p>
            <p><strong>File System:</strong> {driver.file_system}</p>
        </div>
    </div>

    <ProgressBar progress={driver.used_space} max={driver.total_space} />
</button>

<style>
    h3,
    p {
        margin: 0;
    }

    img {
        max-width: 40%;
        max-height: 40%;
        object-fit: cover;
    }

    .card {
        cursor: pointer;
        width: 100%;
        border: 1px solid black;
        padding: 10px 12px;
        border-radius: 8px;
        height: fit-content;
        display: flex;
        flex-direction: column;
        gap: 8px;
    }

    .card-content {
        display: flex;
        align-items: center;
        gap: 20px;
    }

    .info {
        text-align: left;
    }
</style>
