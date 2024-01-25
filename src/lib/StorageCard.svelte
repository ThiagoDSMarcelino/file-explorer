<script lang="ts">
    import ProgressBar from "./ProgressBar.svelte";

    export let storageData: StorageModel;

    const imagePath = "ssd-image.png";

    const cardClicked = () => {
        console.log("Disk selected!");
    };
</script>

<button class="card" on:click={cardClicked}>
    <div class="card-content">
        <img src={imagePath} alt="Storage" />
        <div class="info">
            <h3>{storageData.name} - {storageData.dir}</h3>
            <p><strong>Driver Kind:</strong> {storageData.kind}</p>
            <p><strong>File System:</strong> {storageData.file_system}</p>
        </div>
    </div>

    <ProgressBar
        progress={storageData.used_space}
        max={storageData.total_space}
    />
</button>

<style>
    h3,
    p {
        margin: 0;
    }

    img {
        max-width: 10vw;
        max-height: 10vw;
        object-fit: cover;
    }

    .card {
        cursor: pointer;
        width: 40vw;
        border: 1px solid black;
        padding: 10px 12px;
        margin: 8px 0;
        border-radius: 8px;
        height: fit-content;
        display: flex;
        flex-direction: column;
        gap: 8px;
    }

    .card-content {
        display: flex;
        align-items: center;
        justify-content: space-between;
    }

    .info {
        text-align: left;
    }
</style>
