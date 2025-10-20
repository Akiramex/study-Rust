<script setup lang="ts">
import { loadWasm } from "./wasm/wasmLoader";
import { random } from "./utils/random";
import { World, Direction} from "./wasm/wasm_game";
import { onMounted, ref } from "vue";

const snake_world = ref<HTMLCanvasElement | null>(null);

const whole_world = ref<World | null>(null);

onMounted(async () => {
  

  await createWorld();
});

const handleArrorKey = (event: KeyboardEvent) => {
  if (!whole_world.value) return;
  switch (event.key) {
    case "ArrowUp":
      whole_world.value.change_snake_direction(Direction.Up);
      break;
    case "ArrowDown":
      whole_world.value.change_snake_direction(Direction.Down);
      break;
    case "ArrowLeft":
      whole_world.value.change_snake_direction(Direction.Left);
      break;
    case "ArrowRight":
      whole_world.value.change_snake_direction(Direction.Right);
      break;
  }
  
}

const createWorld = async () => {
  const wasm = await loadWasm()

  const CELL_SIZE = 20;
  const WORLD_WIDTH = 20;
  const fps = 4;

  const snakeIndex = random(WORLD_WIDTH * WORLD_WIDTH);
  whole_world.value = wasm.World.new(WORLD_WIDTH, snakeIndex);
  const worldWidth = whole_world.value.width();

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

  function drawReward() {
    if (!whole_world.value) return;
    const reward_index = whole_world.value.reward_cell();
    const row = Math.floor(reward_index / worldWidth);
    const col = reward_index % worldWidth;

    ctx.beginPath();
    ctx.fillStyle = "red";
    ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    ctx.stroke();
  }

  function drawSnake() {
    if (!whole_world.value) return;
    const snake_index = whole_world.value.snake_head_index();
    const row = Math.floor(snake_index / worldWidth);
    const col = snake_index % worldWidth;

    ctx.beginPath();
    ctx.fillStyle = "green";
    ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    ctx.stroke();
  }

  function run () {
    setTimeout(() => {
      if (!whole_world.value) return;
      whole_world.value.update();
      ctx.clearRect(0, 0, snake_world.value!.width, snake_world.value!.height);
      drawWorld();
      drawSnake();
      drawReward();
      requestAnimationFrame(run);
    }, 1000 / fps);

  }

  run();
}
</script>

<template>
  <div @keydown="handleArrorKey" tabindex="0" class="content-warpper">
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
