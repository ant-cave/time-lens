<template>
  <div class="page-container">
    <h2>今日</h2>
    <p>今日页面占位</p>
    <button @click="getWindowInfo">获取最顶端窗口信息</button>
  </div>
</template>

<script setup>
import { invoke } from '@tauri-apps/api/core'

async function getWindowInfo() {
  try {
    const info = await invoke('get_foreground_window_info')
    if (info) {
      alert(`标题: ${info.title}\n路径: ${info.exe_path}\nPID: ${info.pid}`)
    } else {
      alert('无法获取窗口信息')
    }
  } catch (e) {
    alert('调用失败: ' + e)
  }
}
</script>

<style scoped>
.page-container {
  padding: 24px;
}
button {
  margin-top: 16px;
  padding: 8px 16px;
  cursor: pointer;
}
</style>
