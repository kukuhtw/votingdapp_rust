<!-- frontend/src/pages/admin/PollsList.vue -->
<template>
  <div class="card">
    <h2>Polls</h2>

    <table class="tbl">
      <thead>
        <tr>
          <th>Title</th>
          <th>Status</th>
          <th>On-chain</th>
          <th>Tx Hash</th>
          <th>Aksi</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="p in polls" :key="p.id">
          <td>{{ p.title }}</td>
          <td>{{ p.status }}</td>
          <td>
            <span :class="['badge', p.onchain_status || 'none']">
              {{ p.onchain_status || 'none' }}
            </span>
          </td>

          <td class="tx-cell">
            <template v-if="p.onchain_tx_hash">
              <a
                v-if="explorerBase"
                :href="`${explorerBase}/${p.onchain_tx_hash}`"
                target="_blank" rel="noopener"
              >
                {{ p.onchain_tx_hash }}
              </a>
              <span v-else>{{ p.onchain_tx_hash }}</span>
            </template>
            <span v-else>-</span>
          </td>

          <td>
            <button
              :disabled="isPushing(p.id) || p.onchain_status === 'pending' || p.onchain_status === 'success'"
              @click="push(p)"
              :title="p.onchain_status === 'pending' ? 'Sedang diproses' : 'Kirim create-poll ke chain'"
            >
              <span v-if="isPushing(p.id)">Pushing…</span>
              <span v-else>Push on chain</span>
            </button>
          </td>
        </tr>

        <tr v-if="!polls.length">
          <td colspan="5" style="text-align:center; opacity:.7; padding:1rem;">
            Belum ada poll.
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { Api } from '@/services/api'
import { useAuth } from '@/stores/auth'

const { state } = useAuth()
const polls = ref<any[]>([])
const pushingId = ref<string | null>(null)
const explorerBase = import.meta.env.VITE_EXPLORER_TX || ''

function isPushing(id: string) {
  return pushingId.value === id
}

async function load() {
  const data = await Api.listAdminPolls(state.token)
  polls.value = Array.isArray(data) ? data : []
}

async function push(p: any) {
  if (isPushing(p.id)) return
  try {
    pushingId.value = p.id
    const tx = await Api.pushPollOnchain(state.token, p.id)
    alert(`Pushed! tx=${tx}`)
  } catch (e: any) {
    alert(e?.message || String(e))
  } finally {
    pushingId.value = null
    await load()
  }
}

onMounted(load)
</script>

<style scoped>
.tbl { width: 100%; border-collapse: collapse; }
.tbl th, .tbl td { border-bottom: 1px solid #ddd; padding: .5rem; text-align: left; }
.tx-cell { max-width: 280px; overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }
button[disabled] { opacity: .5; cursor: not-allowed; }

/* badge status (opsional) */
.badge { padding: .15rem .4rem; border-radius: .35rem; font-size: .85em; border: 1px solid transparent; }
.badge.success { background: #e6ffed; border-color: #b7f5c4; }
.badge.pending { background: #fff7e6; border-color: #ffe2a6; }
.badge.failed  { background: #ffecec; border-color: #ffb3b3; }
.badge.none    { background: #f5f5f5; border-color: #e5e5e5; }
</style>
