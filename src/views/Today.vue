<template>
  <div class="page-container">
    <h2>今日</h2>
    
    <div class="window-info-card">
      <h3>当前窗口信息</h3>
      <div v-if="windowInfo" class="info-list">
        <div class="info-item">
          <span class="label">标题：</span>
          <span class="value">{{ windowInfo.title }}</span>
        </div>
        <div class="info-item">
          <span class="label">程序路径：</span>
          <span class="value">{{ windowInfo.exe_path }}</span>
        </div>
        <div class="info-item">
          <span class="label">PID：</span>
          <span class="value">{{ windowInfo.pid }}</span>
        </div>
        <div class="info-item">
          <span class="label">窗口句柄：</span>
          <span class="value">{{ windowInfo.hwnd }}</span>
        </div>
        <div class="info-item">
          <span class="label">可见：</span>
          <span class="value">{{ windowInfo.visible ? '是' : '否' }}</span>
        </div>
        <div class="info-item">
          <span class="label">位置：</span>
          <span class="value">{{ windowInfo.position[0] }}, {{ windowInfo.position[1] }}</span>
        </div>
        <div class="info-item">
          <span class="label">大小：</span>
          <span class="value">{{ windowInfo.size[0] }} × {{ windowInfo.size[1] }}</span>
        </div>
        <div class="info-item">
          <span class="label">状态：</span>
          <span class="value">
            {{ windowInfo.is_maximized ? '最大化' : '' }}
            {{ windowInfo.is_minimized ? '最小化' : '' }}
            {{ !windowInfo.is_maximized && !windowInfo.is_minimized ? '正常' : '' }}
          </span>
        </div>
        <div class="info-item">
          <span class="label">样式：</span>
          <span class="value">{{ windowInfo.style }}</span>
        </div>
        <div class="info-item">
          <span class="label">扩展样式：</span>
          <span class="value">{{ windowInfo.extended_style }}</span>
        </div>
        <div class="info-item">
          <span class="label">父窗口：</span>
          <span class="value">{{ windowInfo.parent_hwnd || '无' }}</span>
        </div>
        <div class="info-item">
          <span class="label">进程启动时间：</span>
          <span class="value">{{ windowInfo.process_start_time ? new Date(windowInfo.process_start_time / 10000 - 11644473600000).toLocaleString() : '未知' }}</span>
        </div>
      </div>
      <div v-else class="no-data">
        <p>点击下方按钮获取当前窗口信息</p>
      </div>
    </div>

    <div class="action-area">
      <button @click="startAutoRefresh" :disabled="isAutoRefreshing">
        {{ isAutoRefreshing ? '自动刷新中...' : '开始自动刷新' }}
      </button>
      <button @click="stopAutoRefresh" :disabled="!isAutoRefreshing">
        停止自动刷新
      </button>
      <button @click="manualRefresh">
        手动刷新
      </button>
    </div>

    <div v-if="lastUpdated" class="last-updated">
      最后更新：{{ lastUpdated }}
    </div>
  </div>
</template>

<script setup>
import { ref, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const windowInfo = ref(null)
const isAutoRefreshing = ref(false)
const autoRefreshInterval = ref(null)
const lastUpdated = ref('')

const clickGetWindowInfo = async () => {
  try {
    const info = await invoke('get_foreground_window_info')
    if (info) {
      windowInfo.value = info
      lastUpdated.value = new Date().toLocaleTimeString()
    } else {
      windowInfo.value = null
      alert('无法获取窗口信息')
    }
  } catch (e) {
    alert('调用失败: ' + e)
  }
}

const startAutoRefresh = () => {
  if (isAutoRefreshing.value) return
  isAutoRefreshing.value = true
  clickGetWindowInfo()
  autoRefreshInterval.value = setInterval(() => {
    clickGetWindowInfo()
  }, 200)
}

const stopAutoRefresh = () => {
  isAutoRefreshing.value = false
  if (autoRefreshInterval.value) {
    clearInterval(autoRefreshInterval.value)
    autoRefreshInterval.value = null
  }
}

const manualRefresh = () => {
  clickGetWindowInfo()
}

onUnmounted(() => {
  stopAutoRefresh()
})
</script>

<style scoped>
.page-container {
  padding: 24px;
}

h2 {
  margin-bottom: 20px;
  color: var(--fgColor-default);
}

.window-info-card {
  background-color: var(--bgColor-muted);
  border-radius: 8px;
  padding: 20px;
  margin-bottom: 20px;
}

.window-info-card h3 {
  margin-top: 0;
  margin-bottom: 16px;
  color: var(--fgColor-default);
}

.info-list {
  display: grid;
  grid-template-columns: auto 1fr;
  gap: 12px;
  align-items: center;
}

.info-item {
  display: flex;
  align-items: center;
}

.label {
  font-weight: bold;
  color: var(--fgColor-muted);
  margin-right: 8px;
  min-width: 80px;
}

.value {
  word-break: break-all;
  color: var(--fgColor-default);
}

.no-data {
  text-align: center;
  padding: 40px 20px;
  color: var(--fgColor-muted);
  font-style: italic;
}

.action-area {
  display: flex;
  gap: 12px;
  margin-bottom: 20px;
  flex-wrap: wrap;
}

button {
  padding: 10px 20px;
  cursor: pointer;
  border: 1px solid var(--borderColor-default);
  border-radius: 6px;
  background-color: var(--bgColor-default);
  color: var(--fgColor-default);
  font-size: 14px;
  transition: all 0.2s;
}

button:hover:not(:disabled) {
  background-color: var(--bgColor-muted);
}

button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.last-updated {
  font-size: 12px;
  color: var(--fgColor-muted);
  text-align: right;
}

@media (max-width: 768px) {
  .info-list {
    grid-template-columns: 1fr;
  }
  
  .action-area {
    flex-direction: column;
  }
  
  button {
    width: 100%;
  }
}
</style>
