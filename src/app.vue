<script setup lang="ts">
import init, { Universe, Cell } from '@/wasm/pkg'

const canvasRef = ref<HTMLCanvasElement>()
const frame = ref(0)
const fps = ref(0)

const CELL_SIZE = 2
const GRID_COLOR = '#CCCCCC'
const DEAD_COLOR = '#FFFFFF'
const ALIVE_COLOR = '#000000'

onMounted(async () => {
  const canvas = canvasRef.value
  if (!canvas) {
    return
  }
  const ctx = canvas.getContext('2d')
  if (!ctx) {
    return
  }

  const drawGrid = () => {
    ctx.beginPath()
    ctx.strokeStyle = GRID_COLOR

    for (let i = 0; i <= width; ++i) {
      const x = i * (CELL_SIZE + 1) + 1
      ctx.moveTo(x, 0)
      ctx.lineTo(x, (CELL_SIZE + 1) * height + 1)
    }

    for (let j = 0; j <= height; ++j) {
      const y = j * (CELL_SIZE + 1) + 1
      ctx.moveTo(0, y)
      ctx.lineTo((CELL_SIZE + 1) * width + 1, y)
    }

    ctx.stroke()
  }

  const drawCells = () => {
    const cellsPtr = universe.cells()
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height)

    ctx.beginPath()

    for (let row = 0; row < height; ++row) {
      for (let col = 0; col < width; ++col) {
        const idx = getIdx(row, col)

        ctx.fillStyle = cells[idx] === Cell.Dead
          ? DEAD_COLOR
          : ALIVE_COLOR

        ctx.fillRect(
          col * (CELL_SIZE + 1) + 1,
          row * (CELL_SIZE + 1) + 1,
          CELL_SIZE,
          CELL_SIZE
        )
      }

      ctx.stroke()
    }
  }

  const getIdx = (r: number, c: number) => {
    return r * width + c
  }

  const renderLoop = () => {
    universe.tick()
    drawCells()
    fps.value = 1000 * frame.value / Math.max(1, Date.now() - start)
    frame.value += 1

    window.setTimeout(() => requestAnimationFrame(renderLoop))
  }

  const { memory } = await init()

  const universe = Universe.new()
  const width = universe.width()
  const height = universe.height()

  canvas.height = (CELL_SIZE + 1) * height + 1
  canvas.width = (CELL_SIZE + 1) * width + 1

  drawGrid()
  const start = Date.now()
  renderLoop()
})
</script>

<template>
  <div>
    <p>frame: {{ frame }} fps: {{ fps }}</p>
    <canvas ref="canvasRef" />
  </div>
</template>
