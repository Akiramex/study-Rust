<script setup lang="ts">
import { loadWasm } from "./wasm/wasmLoader";
import { onMounted, ref } from "vue";

const snake_world = ref<HTMLCanvasElement | null>(null);

onMounted(async () => {
  await createWorld();
});

const createWorld = async () => {
  const wasm = await loadWasm()

  const CELL_SIZE = 20;

  const world = wasm.World.new(20);
  const worldWidth = world.width();

  //const canvas = document.getElementById("snake-world") as HTMLCanvasElement;
  if (!snake_world.value) return;
  const ctx = snake_world.value.getContext("2d") as CanvasRenderingContext2D;

  snake_world.value.width = worldWidth * CELL_SIZE;
  snake_world.value.height = worldWidth * CELL_SIZE;

  function drawWorld() {
    ctx.beginPath();
    for (let x = 0; x < worldWidth + 1; x++) {
      ctx.moveTo(x * CELL_SIZE, 0);
      ctx.lineTo(x * CELL_SIZE, worldWidth * CELL_SIZE);
    }

    for (let y = 0; y < worldWidth + 1; y++) {
      ctx.moveTo(0, y * CELL_SIZE);
      ctx.lineTo(worldWidth * CELL_SIZE, y * CELL_SIZE);
    }
    ctx.stroke();
  }

  drawWorld();
}
</script>

<template>
  <div class="content-warpper">
    hello world
    <canvas id="snake-world" ref ="snake_world">

    </canvas>
  </div>
</template>

<style scoped>
.content-warpper {
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  position: absolute;
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
}
</style>
