/* # frontend/src/components/PollForm.vue

*/
<template>
  <div class="card">
    <div class="row">
      <div class="col">
        <div class="label">Vote Price (micro)</div>
        <input v-model="form.vote_price" placeholder="10000" />
      </div>
    </div>

    <div class="label">Title</div>
    <input v-model="form.title" placeholder="Title" />

    <!-- Slug preview (auto dari title) -->
    <small class="muted">Slug: <code>{{ slug }}</code></small>

    <div class="label">Description</div>
    <textarea v-model="form.description" rows="4" placeholder="Description" />

    <div class="label">Options</div>
    <div v-for="(opt,i) in form.options" :key="i" class="row">
      <input v-model="form.options[i]" placeholder="Option label" />
      <button @click="remove(i)" type="button">Remove</button>
    </div>
    <button @click="add" type="button">+ Add option</button>

    <div class="row" style="margin-top:1rem;">
      <div class="col">
        <div class="label">Start (optional)</div>
        <input
          type="datetime-local"
          v-model="startLocal"
          :max="endLocal || undefined"
          placeholder="Pilih tanggal & waktu mulai"
        />
      </div>
      <div class="col">
        <div class="label">End (required)</div>
        <input
          type="datetime-local"
          v-model="endLocal"
          :min="startLocal || undefined"
          placeholder="Pilih tanggal & waktu selesai"
          required
        />
      </div>
    </div>

    <div style="margin-top:1rem;">
      <button type="button" @click="onSave">Save</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref, watchEffect, computed } from 'vue'

const props = defineProps<{ modelValue?: any }>()
const emit = defineEmits<{ (e: 'submit', payload: any): void }>()

// helpers epoch <-> datetime-local
function epochToLocal(dtSec?: number | null): string {
  if (dtSec == null || dtSec === 0) return ''
  const d = new Date(dtSec * 1000)
  const off = d.getTimezoneOffset()
  const local = new Date(d.getTime() - off * 60000)
  return local.toISOString().slice(0, 16)
}
function localToEpoch(s: string): number | null {
  if (!s) return null
  const d = new Date(s)
  if (Number.isNaN(d.getTime())) return null
  return Math.floor(d.getTime() / 1000)
}

// slugify dari title
function slugify(input: string): string {
  return (input || '')
    .normalize('NFKD')
    .replace(/[\u0300-\u036f]/g, '')
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-+|-+$/g, '')
    .replace(/-+/g, '-')
    .slice(0, 191)
}

const form = reactive({
  title: props.modelValue?.title || '',
  description: props.modelValue?.description || '',
  options: props.modelValue?.options || ['', ''],
  start: props.modelValue?.start ?? null as number | null,
  end: props.modelValue?.end || 0,
  vote_price: props.modelValue?.vote_price || '10000'
})

const slug = computed(() => slugify(form.title))

// UI models for datetime-local
const startLocal = ref<string>(epochToLocal(form.start))
const endLocal = ref<string>(epochToLocal(form.end || null))

watchEffect(() => {
  form.start = localToEpoch(startLocal.value)
  form.end = Number(localToEpoch(endLocal.value) || 0)
})

function add(){ form.options.push('') }
function remove(i:number){ form.options.splice(i,1) }

function onSave() {
  if (!endLocal.value) {
    alert('End datetime wajib diisi.')
    return
  }
  const opts = (form.options || []).map(s => String(s).trim()).filter(Boolean)
  if (opts.length < 2) {
    alert('Minimal 2 option.')
    return
  }
  if (!form.title.trim()) {
    alert('Title wajib diisi.')
    return
  }

  const payload = {
    slug: slug.value,
    title: String(form.title || '').trim(),
    description: String(form.description || '').trim(),
    options: opts,
    start: form.start ?? null,
    end: form.end,
    vote_price: String(form.vote_price ?? '10000').trim()
  }
  emit('submit', payload)
}
</script>

<style scoped>
.row { display:flex; gap:1rem; align-items:flex-start; }
.col { flex:1; }
.label { font-weight:600; margin:0.25rem 0; }
.muted { color:#666; }
input[type="datetime-local"] { width:100%; }
code { background:#f3f3f3; padding:0 .25rem; border-radius:4px; }
</style>
