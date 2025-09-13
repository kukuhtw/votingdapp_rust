
<template>
  <h2>Daftar Poll</h2>
  <div v-if="loading" class="card">Loading...</div>
  <div v-else>
    <div v-for="p in polls" :key="p.id" class="card">
      <h3><router-link :to="'/polls/'+p.slug">{{ p.title }}</router-link></h3>
      <p>{{ p.description }}</p>
      <div><span class="badge">{{ p.status }}</span></div>
    </div>
  </div>
</template>
<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { Api } from '../services/api'

const loading = ref(true)
const polls = ref<any[]>([])

onMounted(async () => {
  try {
    polls.value = await Api.listPublicPolls()
  } finally { loading.value = false }
})
</script>
