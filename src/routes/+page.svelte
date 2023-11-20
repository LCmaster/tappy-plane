<script lang="ts">
    import init from "game";
    import { onMount } from "svelte";

    onMount(async () => {
        await init();
    });
</script>
<canvas id="canvas"></canvas>

