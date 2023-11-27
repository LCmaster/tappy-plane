<script lang="ts"> 
    import { onMount } from "svelte";
    import init from "game";

    onMount(async () => {
        await init();
    });
</script>

    <div class="container mx-auto">
        <div class="flex flex-col justify-center items-center gap-2">
            <h1 
                class="font-sigmar text-4xl text-red-500 drop-shadow-[0_4px_0_rgba(0,0,0,0.75)]"
            >
                Tappy Plane
            </h1>
            <p class="mt-4">
                <strong>Objective:</strong> Maintain the plane flying by clicking/tapping on it, but be aware of the obstacles!
            </p>
            <canvas id="canvas" class="max-w-[800px]" width="800" height="480"></canvas>
        </div>
    </div>

<style>
    canvas {
        cursor: url('/assets/tap.png'), pointer;
    }

    canvas:active {
        cursor: url('/assets/tapTick.png'), grab;
    }
</style>
