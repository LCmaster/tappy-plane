<script lang="ts"> 
    import { onMount } from "svelte";
    import init from "game";

    onMount(async () => {
        await init();
    });
</script>

<canvas id="canvas" width="800" height="480"></canvas>

<style>
    canvas {
        cursor: url('/assets/tap.png'), pointer;
    }

    canvas:active {
        cursor: url('/assets/tapTick.png'), grab;
    }
</style>
