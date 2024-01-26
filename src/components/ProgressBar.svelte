<script>
    export let progress = 0;
    export let max = 100;

    $: style = `width: ${(progress * 100) / max}%;`;
</script>

<div class="progress-bar">
    <div class="progress" {style}></div>

    <div class="tooltip">
        <span>{progress.toFixed(2)} GB</span>
        <span>{max.toFixed(2)} GB</span>
    </div>
</div>

<style>
    .progress-bar {
        background-color: black;
        height: 20px;
        width: 100%;
        padding: 2px;
        position: relative;
    }

    .progress-bar:hover .tooltip {
        display: flex;
    }

    .progress {
        background-color: green;
        height: 100%;
    }

    .tooltip {
        display: none;
        color: white;
        font-size: 14px;
        font-weight: 600;
        justify-content: space-between;
        position: absolute;
        top: 0;
        left: 0;
        padding: 0 5px;
        height: 100%;
        width: calc(100% - 10px);
        align-items: center;
        flex-grow: 1;
    }
</style>
