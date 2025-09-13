
<template><span class="badge">{{ text }}</span></template>
<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from 'vue'
const props = defineProps<{ end: number }>()
const text = ref('')
let t: any
function tick(){
  const now = Math.floor(Date.now()/1000)
  const diff = props.end - now
  if (diff <= 0) { text.value = 'Selesai'; return }
  const d = Math.floor(diff/86400)
  const h = Math.floor((diff%86400)/3600)
  const m = Math.floor((diff%3600)/60)
  text.value = `${d}d ${h}h ${m}m`
}
onMounted(()=>{ tick(); t=setInterval(tick, 1000*30) })
onUnmounted(()=> clearInterval(t))
watch(()=>props.end, tick)
</script>
