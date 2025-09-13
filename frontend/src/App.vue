
<template>
  <div class="app">
    <header class="bar">
      <h1>🗳️ Voting DApp</h1>
      <nav>
        <router-link to="/">Home</router-link>
        <router-link to="/polls">Polls</router-link>
        <router-link to="/admin">Admin</router-link>
      </nav>
    </header>
    <main class="container">
      <router-view />
    </main>
    <footer class="foot">Cosmos • CosmWasm • Vue</footer>
  </div>
</template>

<script setup lang="ts"></script>

<style scoped>
.bar { display:flex; gap:1rem; align-items:center; padding:1rem; border-bottom:1px solid #eee; }
nav { display:flex; gap:1rem; }
.container { padding: 1rem; max-width: 960px; margin: 0 auto; }
.foot { padding:1rem; border-top:1px solid #eee; text-align:center; color:#666; }
</style>
