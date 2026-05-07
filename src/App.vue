<script setup>
import { ref } from 'vue'
import Sidebar from './components/Sidebar.vue'

console.log('hello world')

const sidebarRef = ref(null)
</script>

<template>
  <div class="app-container">
    <Sidebar ref="sidebarRef" />
    <main class="main-content">
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

.app-container {
  display: flex;
  height: 100vh;
  overflow: hidden;
}

.main-content {
  flex: 1;
  overflow: auto;
  height: 100%;
}
</style>
