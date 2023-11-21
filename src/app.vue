<script setup lang="ts">
import init, { Universe } from '@/wasm/pkg'

const canvasRef = ref<HTMLCanvasElement>()
const frame = ref(0)
const fps = ref(0)
const tickTime = ref(0)
const renderTime = ref(0)

const measureFps = () => {
  const interval = 1
  let frame1 = frame.value
  const fn = () => {
    fps.value = (frame.value - frame1) / interval
    frame1 = frame.value
  }
  window.setInterval(fn, interval * 1000)
}

onMounted(async () => {
  const renderLoop = () => {
    const t1 = Date.now()
    universe.tick()
    const t2 = Date.now()
    universe.draw()
    const t3 = Date.now()
    frame.value += 1

    tickTime.value += t2 - t1
    renderTime.value += t3- t2

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
    <p>
      frame: {{ frame }}
      <br>
      fps: {{ fps }}
      <br>
      avg. tick time: {{ tickTime / frame }}
      <br>
      avg. render time: {{ renderTime / frame }}
    </p>
    <canvas ref="canvasRef" id="canvas" />
  </div>
</template>
