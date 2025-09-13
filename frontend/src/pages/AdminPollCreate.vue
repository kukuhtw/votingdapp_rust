
<template>
  <h2>Buat Poll</h2>
  <PollForm @submit="submit" />
</template>
<script setup lang="ts">
import PollForm from '../components/PollForm.vue'
import { Api } from '../services/api'
import { useAuth } from '../stores/auth'
import { useRouter } from 'vue-router'
const { state } = useAuth()
const router = useRouter()
async function submit(payload:any){
  try {
    const id = await Api.createPoll(state.token, payload)
    router.push('/admin/polls')
  } catch(e:any){ alert(e.message || String(e)) }
}
</script>
