<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import Icon from './Icon.vue'

const router = useRouter()
const route = useRoute()

const isSidebarOpen = ref(true)

const toggleSidebar = () => {
  isSidebarOpen.value = !isSidebarOpen.value
}

// 监听窗口大小，竖屏时自动收起
const checkScreenWidth = () => {
  isSidebarOpen.value = window.innerWidth > window.innerHeight
}

const routerLinkPress = () => {
  if (window.innerWidth < window.innerHeight) {
    isSidebarOpen.value = false
  }
}

onMounted(() => {
  checkScreenWidth()
  window.addEventListener('resize', checkScreenWidth)
})

onUnmounted(() => {
  window.removeEventListener('resize', checkScreenWidth)
})

// 暴露给父组件控制
defineExpose({
  isSidebarOpen,
  toggleSidebar
})

// 导航项 - 和 router 对应
// icon 从 Icon.vue 里找现成的
const navItems = [
  { path: '/today', label: '今日', icon: 'eye' },
  { path: '/calendar', label: '日历', icon: 'repo' },
  { path: '/analysis', label: '分析', icon: 'key' },
  { path: '/settings', label: '设置', icon: 'gear' },
]

function isActive(path) {
  return route.path === path
}

async function logout() {
  await invoke('logout')
  router.push('/auth')
}
</script>

<template>
  <aside class="gh-sidebar" :class="{ collapsed: !isSidebarOpen }">
    <div class="gh-sidebar-header">
      <img src="/icon.png" alt="TimeLens" style="width: 32px; height: 32px; border-radius: 6px;" />
      <div>
        <div class="gh-sidebar-title">TimeLens</div>
        <div class="gh-sidebar-subtitle">软件使用时间记录器</div>
      </div>
    </div>
    
    <nav class="gh-sidebar-nav">
      <div class="gh-sidebar-section">
        <div class="gh-sidebar-section-title">功能</div>

        <a
          v-for="item in navItems"
          :key="item.path"
          class="gh-sidebar-item"
          :class="{ active: isActive(item.path) }"
          @click="router.push(item.path); routerLinkPress()"
        >
          <Icon :name="item.icon" class="gh-sidebar-icon" />
          {{ item.label }}
        </a>
      </div>
    </nav>
  </aside>
  <button class="gh-card-action sidebar-toggle" @click="toggleSidebar">
    <span class="sidebar-toggle-icon" :style="{ transform: isSidebarOpen ? 'rotate(0deg)' : 'rotate(180deg)' }" style="transition: all 0.2s ease-in-out;">
      &lt;
    </span>
  </button>
</template>

<style scoped>
.gh-sidebar-item {
  text-decoration: none;
}

.gh-sidebar-item:hover {
  text-decoration: none;
}

.sidebar-toggle {
  height: 30px;
  width: 30px;
  margin: 0;
  position: absolute;
  left: 215px;
  bottom: 16px;
  z-index: 101;
  transition: left 0.4s ease-in-out;
}

.gh-sidebar.collapsed + .sidebar-toggle {
  left: 16px;
}

.sidebar-toggle-icon {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 100%;
  height: 100%;
  transition: transform 0.4s ease-in-out;
}

.gh-sidebar {
  width: 260px;
  height: 100%;
  transition: width, padding 0.35s cubic-bezier(0.4, 0, 0.2, 1);
  overflow: hidden;
  flex-shrink: 0;
  background-color: var(--bgColor-default);
  /* 禁止侧边栏滚动 */
  overscroll-behavior: none;
  scrollbar-width: none;
}

.gh-sidebar::-webkit-scrollbar {
  display: none;
}

.gh-sidebar.collapsed {
  width: 0;
  opacity: 0;
  padding-left: 0px;
  padding-right: 0px;
}

.gh-sidebar {
  transition: all 0.35s cubic-bezier(0.4, 0, 0.2, 1)
}

.gh-sidebar-nav {
  display: flex;
  flex-direction: column;
  height: calc(100% - 64px);
}

@media (orientation: portrait) {
  .gh-sidebar {
    position: absolute;
    top: 0;
    left: 0;
    height: 100%;
    width: 100%;
    z-index: 100;
    /* 默认亮色背景 */
    background-color: #ffffff !important;
    opacity: 1 !important;
  }

  .gh-sidebar:not(.collapsed) {
    width: 100%;
    background-color: #ffffff !important;
    opacity: 1 !important;
  }

  .gh-sidebar.collapsed {
    width: 0;
    opacity: 0 !important;
    pointer-events: none;
  }

  .gh-sidebar.collapsed * {
    opacity: 0;
  }

  .sidebar-toggle {
    left: calc(100% - 46px) !important;
    right: auto !important;
    top: auto !important;
    bottom: 16px !important;
  }

  .gh-sidebar.collapsed + .sidebar-toggle {
    left: 16px !important;
  }

  .gh-sidebar:not(.collapsed) + .sidebar-toggle {
    left: auto !important;
    right: 16px !important;
    z-index: 101;
  }
}

/* 暗色模式 - 根据 data-color-mode 属性 */
@media (orientation: portrait) {
  [data-color-mode="dark"] .gh-sidebar,
  [data-color-mode="dark"] .gh-sidebar:not(.collapsed) {
    background-color: #0d1117 !important;
  }
}
</style>

<style>
/* 非 scoped 样式，确保竖屏模式下侧边栏背景正常 */
@media (orientation: portrait) {
  .gh-sidebar {
    background-color: #ffffff !important;
  }
  
  /* 暗色模式 - 根据 data-color-mode 属性 */
  [data-color-mode="dark"] .gh-sidebar {
    background-color: #0d1117 !important;
  }
}
</style>
