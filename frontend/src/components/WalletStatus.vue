
<template>
  <div class="card">
    <div class="row" style="justify-content:space-between; align-items:center;">
      <div>Wallet: <b>{{ addr || '-' }}</b></div>
      <button @click="connect" v-if="!addr">Connect Wallet</button>
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref } from 'vue'
import { getSigner } from '../services/chain'
const addr = ref<string>('')
async function connect(){
  try {
    const { address } = await getSigner()
    addr.value = address
  } catch(e:any) { alert(e.message || String(e)) }
}
</script>
