
<template>
  <div class="card">
    <div v-for="(opt,i) in options" :key="i" class="row" style="align-items:center;">
      <div style="width:140px">{{ opt }}</div>
      <div style="flex:1; background:#f2f2f2; border-radius:6px; overflow:hidden;">
        <div :style="{ width: pct(i)+'%', height:'16px', background:'#42b883' }"></div>
      </div>
      <div style="width:60px; text-align:right;">{{ counts[i] || 0 }}</div>
    </div>
  </div>
</template>
<script setup lang="ts">
const props = defineProps<{ options: string[], counts: number[] }>()
function pct(i:number){
  const total = (props.counts||[]).reduce((a,b)=>a+b,0) || 1
  return Math.round((props.counts[i] || 0) * 100 / total)
}
</script>
