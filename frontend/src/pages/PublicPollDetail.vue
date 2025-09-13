
<template>
  <div v-if="poll" class="card">
    <h2>{{ poll.title }}</h2>
    <p>{{ poll.description }}</p>
    <div>Berakhir: <Countdown :end="poll.end"/></div>
  </div>

  <WalletStatus />

  <div class="card" v-if="poll">
    <h3>Pilih opsi</h3>
    <div v-for="(opt,i) in poll.options" :key="i" class="row">
      <div class="col">{{ opt }}</div>
      <button @click="vote(i)" :disabled="voting || voted">Vote</button>
    </div>
    <div v-if="voted" class="badge">Anda sudah voting</div>
  </div>

  <ResultChart v-if="result" :options="poll?.options||[]" :counts="result.counts"/>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useRoute } from 'vue-router'
import Countdown from '../components/Countdown.vue'
import WalletStatus from '../components/WalletStatus.vue'
import ResultChart from '../components/ResultChart.vue'
import { voteOnce } from '../services/cw20'
import { Api } from '../services/api'

const route = useRoute()
const poll = ref<any>(null)
const result = ref<{counts:number[]}|null>(null)
const voting = ref(false)
const voted = ref(false)

async function fetchPoll() {
  // Demo: try load public list then find by slug
  const list:any[] = await Api.listPublicPolls()
  const found = list.find(x => x.slug === route.params.slug)
  poll.value = found || list[0] || {
    poll_id: 'poll-demo',
    slug: 'demo',
    title: 'Contoh Poll',
    description: 'Pilih prioritas.',
    options: ['Transportasi','Kesehatan','Pendidikan'],
    end: Math.floor(Date.now()/1000) + 3600*24*2,
    vote_price: '10000'
  }
  result.value = { counts: [10, 12, 4] }
}

async function vote(i:number){
  try { voting.value = true } catch {}
  try {
    await voteOnce(poll.value.poll_id || poll.value.slug, i, poll.value.vote_price)
    voted.value = true
    if (result.value) result.value.counts[i]++
  } catch(e:any) {
    alert(e.message || String(e))
  } finally {
    voting.value = false
  }
}

onMounted(fetchPoll)
</script>
