<script lang="ts">
    import { onMount } from "svelte";

    import init from "game";

    onMount(async () => {
            console.log("Game Init");
            await init();
    });
</script>

<canvas id="canvas" width="640" height="480"></canvas>

