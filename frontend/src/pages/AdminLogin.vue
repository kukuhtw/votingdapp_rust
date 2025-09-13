
<template>
  <div class="card">
    <h2>Admin Login</h2>
    <div class="label">Email</div>
    <input v-model="email" placeholder="admin@example.com"/>
    <div class="label">Password</div>
    <input v-model="password" type="password" placeholder="••••••••"/>
    <div style="margin-top:1rem;">
      <button @click="login" :disabled="loading">Login</button>
    </div>
  </div>
</template>
<script setup lang="ts">
import { ref } from 'vue'
import { Api } from '../services/api'
import { useAuth } from '../stores/auth'
import { useRouter } from 'vue-router'

const email = ref('admin@example.com')
const password = ref('changeme')
const loading = ref(false)
const router = useRouter()
const { setToken } = useAuth()

async function login(){
  loading.value = true
  try {
    const r = await Api.login(email.value, password.value)
    setToken(r.token)
    router.push('/admin')
  } catch(e:any) { alert(e.message || String(e)) }
  finally { loading.value = false }
}
</script>
