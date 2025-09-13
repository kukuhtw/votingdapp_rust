
<template>
  <h2>Publish Poll</h2>
  <div class="card" v-if="poll">
    <div><b>{{ poll.title }}</b></div>
    <div>{{ poll.description }}</div>
    <div>Options: {{ poll.options.join(', ') }}</div>
    <div>End: {{ poll.end }}</div>
    <div>Vote Price: {{ poll.vote_price }}</div>
    <button @click="publish" :disabled="working">Publish to Chain</button>
  </div>
</template>
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { Api } from '../services/api'
import { useAuth } from '../stores/auth'
import { useRoute } from 'vue-router'
import { getSigner } from '../services/chain'

const route = useRoute()
const { state } = useAuth()
const poll = ref<any>(null)
const working = ref(false)

onMounted(async ()=>{
  poll.value = await Api.getPoll(state.token, route.params.id as string)
})

async function publish(){
  working.value = true
  try {
    const { address } = await getSigner()
    alert(`Signed by ${address}. Implement on-chain create_poll here.`)
    // Setelah sukses on-chain, panggil endpoint backend publish-callback di sini.
  } catch(e:any){ alert(e.message || String(e)) }
  finally { working.value = false }
}
</script>
