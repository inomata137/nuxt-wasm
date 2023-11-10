<script setup lang="ts">
import init, { greet } from '@/wasm/pkg'

onMounted(async () => {
  await init()
  greet('Wasm via Nuxt3')
})
</script>

<template>
  <div>
    <NuxtWelcome />
  </div>
</template>
