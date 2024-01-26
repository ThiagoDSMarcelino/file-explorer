<script>
    export let progress = 0;
    export let max = 100;

    $: style = `width: ${max - progress}%;`;
</script>

<div class="progress-bar">
    <div class="progress" {style}></div>

    <div class="tooltip">
        <span>{progress.toFixed(2)}</span>
        <span>{max.toFixed(2)}</span>
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

    .progress {
        background-color: green;
        height: 100%;
    }

    .tooltip {
        display: flex;
        color: white;
        font-size: 14px;
        font-weight: bold;
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
