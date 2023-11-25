<script setup lang="ts">
import init, { Universe } from '@/wasm/pkg'

const canvasRef = ref<HTMLCanvasElement>()
const frame = ref(0)
const fps = ref(0)

const measureFps = () => {
  let frame1 = frame.value
  window.setInterval(() => {
    fps.value = frame.value - frame1
    frame1 = frame.value
  }, 1000)
}

onMounted(async () => {
  const renderLoop = () => {
    universe.tick()
    frame.value += 1

    requestAnimationFrame(renderLoop)
  }

  await init()
  const universe = Universe.new('canvas')
  renderLoop()
  measureFps()
})
</script>

<template>
  <div>
    <p>fps: {{ fps }}</p>
    <canvas ref="canvasRef" id="canvas" />
  </div>
</template>
