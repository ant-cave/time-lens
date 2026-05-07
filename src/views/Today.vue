<template>
  <div class="page-container">
    <div class="page-header">
      <h2>今日</h2>
      <span class="today-date">{{ todayDate }}</span>
    </div>

    <!-- Current Activity Card -->
    <div class="current-card" :class="{ idle: !currentSession }">
      <div class="current-indicator">
        <span class="recording-dot" :class="{ active: !!currentSession }"></span>
        <span class="status-text">{{ currentSession ? '正在使用' : '无活动窗口' }}</span>
      </div>

      <div class="current-app" v-if="currentSession">
        <div class="app-icon-placeholder">{{ currentSession.app_name.charAt(0) }}</div>
        <div class="app-details">
          <div class="app-name">{{ currentSession.app_name }}</div>
          <div class="window-title">{{ currentSession.window_title }}</div>
        </div>
      </div>

      <div class="current-app idle-state" v-else>
        <div class="app-icon-placeholder idle">--</div>
        <div class="app-details">
          <div class="app-name idle-text">未检测到活动</div>
          <div class="window-title">桌面或锁屏状态</div>
        </div>
      </div>

      <div class="current-timer">
        <div class="timer-value">{{ formatTime(currentElapsed) }}</div>
        <div class="timer-label">已用时间</div>
      </div>
    </div>

    <!-- Stats Overview -->
    <div class="stats-row">
      <div class="stat-box">
        <div class="stat-value">{{ totalApps }}</div>
        <div class="stat-label">应用数</div>
      </div>
      <div class="stat-box">
        <div class="stat-value">{{ formatHours(totalTime) }}</div>
        <div class="stat-label">总时长</div>
      </div>
      <div class="stat-box">
        <div class="stat-value">{{ currentSession ? sessionCount + 1 : sessionCount }}</div>
        <div class="stat-label">会话数</div>
      </div>
    </div>

    <!-- Chart & List -->
    <div class="content-grid">
      <div class="card chart-card">
        <h3>使用分布</h3>
        <div class="chart-wrapper" v-if="aggregated.length > 0">
          <canvas ref="chartCanvas"></canvas>
        </div>
        <div class="no-chart-data" v-else>
          <p>今日暂无记录</p>
          <p class="hint">开始使用应用后，数据将在此显示</p>
        </div>
      </div>

      <div class="card list-card">
        <h3>应用详情</h3>
        <div class="app-list" v-if="aggregated.length > 0">
          <div
            v-for="(item, index) in aggregated"
            :key="item.app_name"
            class="app-list-item"
          >
            <span class="app-rank" :style="{ color: chartColors[index % chartColors.length] }">
              {{ index + 1 }}
            </span>
            <span class="app-color-dot" :style="{ background: chartColors[index % chartColors.length] }"></span>
            <span class="app-list-name">{{ item.app_name }}</span>
            <span class="app-list-time">{{ formatTime(item.total_seconds) }}</span>
            <div class="app-bar-bg">
              <div
                class="app-bar-fill"
                :style="{ width: (item.total_seconds / maxTime * 100) + '%', background: chartColors[index % chartColors.length] }"
              ></div>
            </div>
          </div>
        </div>
        <div class="no-list-data" v-else>
          <p>暂无使用记录</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, nextTick, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { Chart, registerables } from 'chart.js'

Chart.register(...registerables)

const todayDate = ref('')
const currentSession = ref(null)
const currentElapsed = ref(0)
const aggregated = ref([])
const totalApps = ref(0)
const totalTime = ref(0)
const sessionCount = ref(0)
const maxTime = ref(1)

const chartCanvas = ref(null)
let chartInstance = null
let timerInterval = null
let unlistenAppChanged = null
let unlistenAppUpdate = null
let unlistenAggregated = null

const chartColors = [
  '#4F46E5', '#0EA5E9', '#10B981', '#F59E0B', '#EF4444',
  '#8B5CF6', '#EC4899', '#14B8A6', '#F97316', '#6366F1',
  '#84CC16', '#06B6D4', '#D946EF', '#22C55E', '#EAB308',
]

function formatTime(seconds) {
  if (!seconds || seconds <= 0) return '0s'
  const h = Math.floor(seconds / 3600)
  const m = Math.floor((seconds % 3600) / 60)
  const s = seconds % 60
  let result = ''
  if (h > 0) result += `${h}h `
  if (m > 0 || h > 0) result += `${m}m `
  result += `${s}s`
  return result
}

function formatHours(seconds) {
  if (!seconds || seconds <= 0) return '0h'
  const h = (seconds / 3600).toFixed(1)
  return `${h}h`
}

async function loadAggregated() {
  try {
    const data = await invoke('get_today_aggregated')
    aggregated.value = data || []
    totalApps.value = data ? data.length : 0
    totalTime.value = data ? data.reduce((sum, item) => sum + item.total_seconds, 0) : 0
    sessionCount.value = data ? data.length : 0
    maxTime.value = data && data.length > 0 ? Math.max(...data.map(i => i.total_seconds)) : 1
    updateChart()
  } catch (e) {
    console.error('Failed to load aggregated data:', e)
  }
}

async function refreshAll() {
  await loadAggregated()
  if (currentSession.value) {
    const now = Math.floor(Date.now() / 1000)
    const start = currentSession.value.start_time
    currentElapsed.value = now - start
  }
}

function updateChart() {
  if (!chartCanvas.value) return
  if (!aggregated.value || aggregated.value.length === 0) {
    if (chartInstance) {
      chartInstance.destroy()
      chartInstance = null
    }
    return
  }

  const labels = aggregated.value.map(i => i.app_name)
  const data = aggregated.value.map(i => i.total_seconds)
  const colors = aggregated.value.map((_, i) => chartColors[i % chartColors.length])

  if (chartInstance) {
    chartInstance.data.labels = labels
    chartInstance.data.datasets[0].data = data
    chartInstance.data.datasets[0].backgroundColor = colors
    chartInstance.update()
  } else {
    chartInstance = new Chart(chartCanvas.value, {
      type: 'doughnut',
      data: {
        labels,
        datasets: [{
          data,
          backgroundColor: colors,
          borderWidth: 2,
          borderColor: '#ffffff',
        }]
      },
      options: {
        responsive: true,
        maintainAspectRatio: true,
        cutout: '60%',
        plugins: {
          legend: {
            position: 'bottom',
            labels: {
              boxWidth: 12,
              padding: 12,
              font: { size: 12 }
            }
          },
          tooltip: {
            callbacks: {
              label: function (context) {
                const total = context.dataset.data.reduce((a, b) => a + b, 0)
                const pct = ((context.parsed / total) * 100).toFixed(1)
                return ` ${context.label}: ${formatTime(context.parsed)} (${pct}%)`
              }
            }
          }
        }
      }
    })
  }
}

function startTimer() {
  stopTimer()
  currentElapsed.value = 0
  const start = currentSession.value ? currentSession.value.start_time : Math.floor(Date.now() / 1000)
  const now = Math.floor(Date.now() / 1000)
  currentElapsed.value = now - start

  timerInterval = setInterval(() => {
    refreshAll()
  }, 1000)
}

function stopTimer() {
  if (timerInterval) {
    clearInterval(timerInterval)
    timerInterval = null
  }
}

onMounted(async () => {
  const now = new Date()
  todayDate.value = `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}-${String(now.getDate()).padStart(2, '0')}`

  await loadAggregated()

  // Listen for events
  unlistenAppChanged = await listen('app-changed', (event) => {
    currentSession.value = event.payload
    startTimer()
  })

  unlistenAppUpdate = await listen('app-update', (event) => {
    const payload = event.payload
    if (payload) {
      currentSession.value = payload
      currentElapsed.value = payload.elapsed_seconds
    }
  })

  unlistenAggregated = await listen('today-aggregated', (event) => {
    const data = event.payload
    if (data && Array.isArray(data)) {
      aggregated.value = data
      totalApps.value = data.length
      totalTime.value = data.reduce((sum, item) => sum + item.total_seconds, 0)
      sessionCount.value = data.length
      maxTime.value = data.length > 0 ? Math.max(...data.map(i => i.total_seconds)) : 1
      nextTick(() => updateChart())
    }
  })
})

onUnmounted(() => {
  stopTimer()
  if (chartInstance) {
    chartInstance.destroy()
    chartInstance = null
  }
  if (unlistenAppChanged) unlistenAppChanged()
  if (unlistenAppUpdate) unlistenAppUpdate()
  if (unlistenAggregated) unlistenAggregated()
})
</script>

<style scoped>
.page-container {
  padding: 24px;
  max-width: 1000px;
  margin: 0 auto;
}

.page-header {
  display: flex;
  align-items: baseline;
  gap: 12px;
  margin-bottom: 20px;
}

.page-header h2 {
  margin: 0;
  font-size: 24px;
  font-weight: 600;
  color: var(--fgColor-default, #1f2328);
}

.today-date {
  font-size: 14px;
  color: var(--fgColor-muted, #656d76);
}

/* Current Activity Card */
.current-card {
  background: var(--bgColor-default, #fff);
  border: 1px solid var(--borderColor-default, #d1d9e0);
  border-radius: 12px;
  padding: 20px 24px;
  margin-bottom: 20px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  box-shadow: 0 1px 3px rgba(0,0,0,0.04);
}

.current-card.idle {
  opacity: 0.7;
}

.current-indicator {
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 100px;
}

.recording-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #d1d9e0;
  transition: all 0.3s;
}

.recording-dot.active {
  background: #10B981;
  box-shadow: 0 0 6px rgba(16, 185, 129, 0.5);
  animation: pulse-dot 2s infinite;
}

@keyframes pulse-dot {
  0%, 100% { box-shadow: 0 0 4px rgba(16, 185, 129, 0.4); }
  50% { box-shadow: 0 0 10px rgba(16, 185, 129, 0.7); }
}

.status-text {
  font-size: 13px;
  font-weight: 500;
  color: var(--fgColor-muted, #656d76);
}

.current-app {
  display: flex;
  align-items: center;
  gap: 14px;
  flex: 1;
}

.app-icon-placeholder {
  width: 44px;
  height: 44px;
  border-radius: 10px;
  background: linear-gradient(135deg, #4F46E5, #6366F1);
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 18px;
  font-weight: 700;
  flex-shrink: 0;
}

.app-icon-placeholder.idle {
  background: var(--bgColor-muted, #f6f8fa);
  color: var(--fgColor-muted, #656d76);
  font-size: 14px;
}

.app-details {
  min-width: 0;
}

.app-name {
  font-size: 16px;
  font-weight: 600;
  color: var(--fgColor-default, #1f2328);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.app-name.idle-text {
  color: var(--fgColor-muted, #656d76);
}

.window-title {
  font-size: 13px;
  color: var(--fgColor-muted, #656d76);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 300px;
}

.current-timer {
  text-align: right;
  flex-shrink: 0;
}

.timer-value {
  font-size: 28px;
  font-weight: 700;
  font-variant-numeric: tabular-nums;
  color: var(--fgColor-default, #1f2328);
  letter-spacing: 1px;
}

.timer-label {
  font-size: 12px;
  color: var(--fgColor-muted, #656d76);
  margin-top: 2px;
}

/* Stats Row */
.stats-row {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 12px;
  margin-bottom: 20px;
}

.stat-box {
  background: var(--bgColor-default, #fff);
  border: 1px solid var(--borderColor-default, #d1d9e0);
  border-radius: 10px;
  padding: 16px;
  text-align: center;
  box-shadow: 0 1px 3px rgba(0,0,0,0.04);
}

.stat-value {
  font-size: 24px;
  font-weight: 700;
  color: var(--fgColor-default, #1f2328);
}

.stat-label {
  font-size: 12px;
  color: var(--fgColor-muted, #656d76);
  margin-top: 4px;
}

/* Content Grid */
.content-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.card {
  background: var(--bgColor-default, #fff);
  border: 1px solid var(--borderColor-default, #d1d9e0);
  border-radius: 12px;
  padding: 20px;
  box-shadow: 0 1px 3px rgba(0,0,0,0.04);
}

.card h3 {
  font-size: 15px;
  font-weight: 600;
  color: var(--fgColor-default, #1f2328);
  margin: 0 0 16px 0;
  padding-bottom: 12px;
  border-bottom: 1px solid var(--borderColor-muted, #d1d9e0);
}

/* Chart */
.chart-wrapper {
  display: flex;
  justify-content: center;
  padding: 8px 0;
  max-height: 300px;
}

.no-chart-data {
  text-align: center;
  padding: 40px 20px;
  color: var(--fgColor-muted, #656d76);
}

.no-chart-data .hint {
  font-size: 12px;
  margin-top: 8px;
}

/* App List */
.app-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.app-list-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 0;
  border-bottom: 1px solid var(--borderColor-muted, #f0f0f0);
}

.app-list-item:last-child {
  border-bottom: none;
}

.app-rank {
  font-size: 12px;
  font-weight: 700;
  width: 20px;
  text-align: center;
  flex-shrink: 0;
}

.app-color-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.app-list-name {
  flex: 1;
  font-size: 13px;
  font-weight: 500;
  color: var(--fgColor-default, #1f2328);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.app-list-time {
  font-size: 12px;
  font-weight: 600;
  color: var(--fgColor-muted, #656d76);
  flex-shrink: 0;
  min-width: 60px;
  text-align: right;
}

.app-bar-bg {
  width: 60px;
  height: 6px;
  background: var(--bgColor-muted, #f6f8fa);
  border-radius: 3px;
  overflow: hidden;
  flex-shrink: 0;
}

.app-bar-fill {
  height: 100%;
  border-radius: 3px;
  transition: width 0.5s ease;
}

.no-list-data {
  text-align: center;
  padding: 30px;
  color: var(--fgColor-muted, #656d76);
}

@media (max-width: 768px) {
  .content-grid {
    grid-template-columns: 1fr;
  }

  .current-card {
    flex-direction: column;
    text-align: center;
  }

  .current-app {
    justify-content: center;
  }

  .current-timer {
    text-align: center;
  }

  .timer-value {
    font-size: 24px;
  }

  .stats-row {
    grid-template-columns: repeat(3, 1fr);
  }
}

body {
  background: var(--bgColor-default, #fff);
}
</style>
