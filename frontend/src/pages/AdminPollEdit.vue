
<template>
  <h2>Edit Poll</h2>
  <div v-if="poll">
    <PollForm :modelValue="poll" @submit="save" />
  </div>
</template>
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import PollForm from '../components/PollForm.vue'
import { Api } from '../services/api'
import { useAuth } from '../stores/auth'
import { useRoute } from 'vue-router'

const route = useRoute()
const { state } = useAuth()
const poll = ref<any>(null)

onMounted(async ()=>{
  poll.value = await Api.getPoll(state.token, route.params.id as string)
})

async function save(payload:any){
  await Api.updatePoll(state.token, route.params.id as string, payload)
  alert('Saved')
}
</script>
