<script setup>
import { ref } from 'vue'
import Sidebar from './components/Sidebar.vue'

const sidebarRef = ref(null)
</script>

<template>
  <div style="display: flex; height: 100vh; overflow: hidden;">
    <Sidebar ref="sidebarRef" />
    <main style="flex: 1; overflow: auto;">
      <router-view />
    </main>
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html,
body {
  height: 100%;
  width: 100%;
  overflow: hidden;
}
</style>
