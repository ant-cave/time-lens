<script setup>
import { ref, onMounted } from "vue";

const isDark = ref(false);

onMounted(() => {
  const html = document.documentElement;
  isDark.value = html.getAttribute("data-color-mode") === "dark";
});

function toggleTheme() {
  const html = document.documentElement;
  const newMode = isDark.value ? "light" : "dark";
  html.setAttribute("data-color-mode", newMode);
  isDark.value = !isDark.value;
}
</script>

<template>
  <main class="container-xl px-6 py-6">
    <div class="Box p-6">
      <div class="d-flex flex-justify-between flex-items-center mb-6">
        <h1 class="h1-mktg mb-0">Primer CSS 样式展示</h1>
        <button class="btn" @click="toggleTheme">
          <svg v-if="!isDark" class="octicon octicon-sun mr-2" viewBox="0 0 16 16" width="16" height="16"><path fill="currentColor" d="M8 1a1 1 0 100-2 1 1 0 000 2zM8 4a1 1 0 100-2 1 1 0 000 2zm0 10a1 1 0 100-2 1 1 0 000 2zM8 7a1 1 0 100-2 1 1 0 000 2zm0 7a1 1 0 100-2 1 1 0 000 2zM4.22 4.22a1 1 0 100-1.44 1 1 0 000 1.44zm0 8.48a1 1 0 100-1.44 1 1 0 000 1.44zm8.48 0a1 1 0 100-1.44 1 1 0 000 1.44zm0-8.48a1 1 0 100-1.44 1 1 0 000 1.44zM1 8a1 1 0 100-2 1 1 0 000 2zm14 0a1 1 0 100-2 1 1 0 000 2zM8 11a3 3 0 110-6 3 3 0 010 6z"/></svg>
          <svg v-else class="octicon octicon-moon mr-2" viewBox="0 0 16 16" width="16" height="16"><path fill="currentColor" d="M9.598 1.362a7.504 7.504 0 00-8.234 9.966 7.502 7.502 0 009.966-8.234.753.753 0 00-.924-.924 4.994 4.994 0 01-5.466-5.466.753.753 0 00.924-.924 7.504 7.504 0 008.234 9.966.753.753 0 00-.924-.924 4.994 4.994 0 01-5.466-5.466.753.753 0 00.924-.924z"/></svg>
          {{ isDark ? '亮色模式' : '暗色模式' }}
        </button>
      </div>
      
      <div class="mb-6">
        <h2 class="h2-mktg mb-3">按钮</h2>
        <div class="BtnGroup mr-2 mb-2">
          <button class="btn BtnGroup-item">默认</button>
          <button class="btn btn-primary BtnGroup-item">主要</button>
          <button class="btn btn-success BtnGroup-item">成功</button>
          <button class="btn btn-danger BtnGroup-item">危险</button>
        </div>
        <div class="mt-2">
          <button class="btn btn-sm">小按钮</button>
          <button class="btn">普通按钮</button>
          <button class="btn btn-large">大按钮</button>
        </div>
      </div>

      <div class="mb-6">
        <h2 class="h2-mktg mb-3">表单</h2>
        <div class="form-group mb-3">
          <label class="form-label">用户名</label>
          <input type="text" class="form-control" placeholder="请输入用户名" />
        </div>
        <div class="form-group mb-3">
          <label class="form-label">邮箱</label>
          <input type="email" class="form-control" placeholder="name@example.com" />
        </div>
        <div class="form-group mb-3">
          <label class="form-label">个人简介</label>
          <textarea class="form-control" rows="3" placeholder="介绍一下你自己..."></textarea>
        </div>
      </div>

      <div class="mb-6">
        <h2 class="h2-mktg mb-3">警告框</h2>
        <div class="flash flash-success mb-2">
          <svg class="octicon octicon-check mr-2" viewBox="0 0 16 16" width="16" height="16"><path fill="currentColor" d="M13.78 4.22a.75.75 0 010 1.06l-7.25 7.25a.75.75 0 01-1.06 0L2.22 9.28a.751.751 0 01.018-1.042.751.751 0 011.042-.018L6 11.06l6.72-6.72a.75.75 0 011.06 0z"/></svg>
          操作成功完成！
        </div>
        <div class="flash flash-warn mb-2">
          <svg class="octicon octicon-alert mr-2" viewBox="0 0 16 16" width="16" height="16"><path fill="currentColor" d="M6.457 1.047c.659-1.234 2.427-1.234 3.086 0l6.082 11.378A1.75 1.75 0 0114.082 15H1.918a1.75 1.75 0 01-1.543-2.575L6.457 1.047zM9 11a1 1 0 11-2 0 1 1 0 012 0zm-.25-5.25a.75.75 0 011.5 0v2.5a.75.75 0 01-1.5 0v-2.5z"/></svg>
          请注意，此操作不可逆。
        </div>
        <div class="flash flash-error">
          <svg class="octicon octicon-x mr-2" viewBox="0 0 16 16" width="16" height="16"><path fill="currentColor" d="M3.72 3.72a.75.75 0 011.06 0L8 6.94l3.22-3.22a.75.75 0 111.06 1.06L9.06 8l3.22 3.22a.75.75 0 11-1.06 1.06L8 9.06l-3.22 3.22a.75.75 0 01-1.06-1.06L6.94 8 3.72 4.78a.75.75 0 010-1.06z"/></svg>
          发生错误，请稍后重试。
        </div>
      </div>

      <div class="mb-6">
        <h2 class="h2-mktg mb-3">GitHub 风格卡片</h2>
        <div class="gh-card mb-3">
          <div class="gh-card-title">创建你的第一个代码项目</div>
          <div class="gh-card-description">在 GitHub 上创建一个项目。这是你自己的空间，可以尝试、学习，并与世界分享代码。</div>
          <button class="gh-card-action">
            <svg class="octicon" viewBox="0 0 16 16" width="12" height="12"><path fill="currentColor" d="M8 0a8 8 0 100 16A8 8 0 008 0zm0 14.5a6.5 6.5 0 110-13 6.5 6.5 0 010 13z"/></svg>
            创建项目
          </button>
        </div>
        <div class="gh-card">
          <div class="gh-card-title">开始学习 GitHub</div>
          <div class="gh-card-description">无论你是编程新手还是想提升版本控制技能，都能帮助你掌握 GitHub 的基础知识。</div>
          <button class="gh-card-action">开始播放列表</button>
        </div>
      </div>

      <div class="mb-6">
        <h2 class="h2-mktg mb-3">GitHub 风格顶部导航栏</h2>
        <div style="border: 1px solid var(--borderColor-default, #d1d9e0); border-radius: 6px; overflow: hidden;">
          <header class="gh-header">
            <div class="gh-header-left">
              <div class="gh-header-logo">
                <svg height="32" aria-hidden="true" viewBox="0 0 16 16" version="1.1" width="32" data-view-component="true" fill="currentColor" style="color: var(--fgColor-default, #1f2328);">
                  <path d="M8 0c4.42 0 8 3.58 8 8a8.013 8.013 0 0 1-5.45 7.59c-.4.08-.55-.17-.55-.38 0-.27.01-1.13.01-2.2 0-.75-.25-1.23-.54-1.48 1.78-.2 3.65-.88 3.65-3.95 0-.88-.31-1.59-.82-2.15.08-.2.36-1.02-.08-2.12 0 0-.67-.22-2.2.82-.64-.18-1.32-.27-2-.27-.68 0-1.36.09-2 .27-1.53-1.03-2.2-.82-2.2-.82-.44 1.1-.16 1.92-.08 2.12-.51.56-.82 1.28-.82 2.15 0 3.06 1.86 3.75 3.64 3.95-.23.2-.44.55-.51 1.07-.46.21-1.61.55-2.33-.66-.15-.24-.6-.83-1.23-.82-.67.01-.27.38.01.53.34.19.73.9.82 1.13.16.45.68 1.31 2.69.94 0 .67.01 1.3.01 1.49 0 .21-.15.45-.55.38A7.995 7.995 0 0 1 0 8c0-4.42 3.58-8 8-8Z"></path>
                </svg>
              </div>
              <div class="gh-header-search">
                <svg class="gh-header-search-icon" viewBox="0 0 16 16" fill="currentColor"><path d="M10.68 11.74a6 6 0 0 1-7.922-8.982 6 6 0 0 1 8.982 7.922l3.04 3.04a.749.749 0 0 1-.326 1.275.749.749 0 0 1-.734-.215ZM11 6.5a4.499 4.499 0 1 0-8.997 0A4.499 4.499 0 0 0 11 6.5Z"/></svg>
                <input type="text" placeholder="Type / to search" />
              </div>
            </div>
            <div class="gh-header-right">
              <button class="gh-header-btn">
                <svg viewBox="0 0 16 16" width="16" height="16" fill="currentColor"><path d="M8 16a2 2 0 0 0 1.985-1.75c.017-.137-.097-.25-.235-.25h-3.5c-.138 0-.252.113-.235.25A2 2 0 0 0 8 16ZM3 5a5 5 0 0 1 10 0v2.947c0 .05.015.098.042.139l1.703 2.555A1.519 1.519 0 0 1 13.482 13H2.518a1.519 1.519 0 0 1-1.263-2.36l1.703-2.554A.255.255 0 0 0 3 7.947Z"/></svg>
              </button>
              <button class="gh-header-btn">
                <svg viewBox="0 0 16 16" width="16" height="16" fill="currentColor"><path d="M1.75 2.5a.75.75 0 0 0 0 1.5h6.5a.75.75 0 0 0 0-1.5h-6.5Zm4 5a.75.75 0 0 0 0 1.5h6.5a.75.75 0 0 0 0-1.5h-6.5Zm0 5a.75.75 0 0 0 0 1.5h6.5a.75.75 0 0 0 0-1.5h-6.5ZM8 1a1.75 1.75 0 1 0 0 3.5A1.75 1.75 0 0 0 8 1Zm4.25 5a1.75 1.75 0 1 0 0 3.5 1.75 1.75 0 0 0 0-3.5Zm-8.5 5a1.75 1.75 0 1 0 0 3.5 1.75 1.75 0 0 0 0-3.5Z"/></svg>
              </button>
              <img class="gh-header-avatar" src="https://github.com/github.png" alt="@user" width="32" height="32" />
            </div>
          </header>
        </div>
      </div>

      <div class="mb-6">
        <h2 class="h2-mktg mb-3">GitHub 风格侧边栏</h2>
        <div style="display: flex; height: 400px; border: 1px solid var(--borderColor-default, #d1d9e0); border-radius: 6px; overflow: hidden;">
          <aside class="gh-sidebar" style="min-height: auto;">
            <div class="gh-sidebar-header">
              <img class="gh-sidebar-logo" src="https://github.com/github.png" alt="@user" />
              <div>
                <div class="gh-sidebar-title">蚁穴011420</div>
                <div class="gh-sidebar-subtitle">Your personal account</div>
              </div>
            </div>
            <nav class="gh-sidebar-nav">
              <div class="gh-sidebar-section">
                <div class="gh-sidebar-section-title">账户</div>
                <a class="gh-sidebar-item active">
                  <svg class="gh-sidebar-icon" viewBox="0 0 16 16" fill="currentColor"><path d="M8 0a8 8 0 1 1 0 16A8 8 0 0 1 8 0ZM5.78 8.75a9.64 9.64 0 0 0 1.9 3.48c.383-.477.78-1.102 1.1-1.947a20.2 20.2 0 0 0-1.1-1.947 9.64 9.64 0 0 0-1.9 3.48Zm5.024-.932a11.4 11.4 0 0 1 2.295 3.6H11.1c-.22-.92-.54-1.768-.94-2.532.4-.764.72-1.612.94-2.532Zm-.94-2.532a11.4 11.4 0 0 1 .94-2.532h2.997a11.4 11.4 0 0 1-2.295 3.6c-.4-.764-.72-1.612-.94-2.532ZM11.1 8.75h2.997a11.4 11.4 0 0 1-2.295 3.6 11.4 11.4 0 0 1-.94-2.532c.4-.764.72-1.612.94-2.532Zm-1.1-4.5a9.64 9.64 0 0 0-1.9-3.48c-.383.477-.78 1.102-1.1 1.947.32.845.717 1.47 1.1 1.947a9.64 9.64 0 0 0 1.9-3.48Zm-2.024.932a11.4 11.4 0 0 1-.94 2.532H4.14a11.4 11.4 0 0 1 2.295-3.6c.4.764.72 1.612.94 2.532Zm-.94 2.532c.22.92.54 1.768.94 2.532a11.4 11.4 0 0 1-2.295-3.6H7.14c.22.92.54 1.768.94 2.532ZM8 1.75a7.25 7.25 0 1 0 0 14.5 7.25 7.25 0 0 0 0-14.5Z"/></svg>
                  公众形象
                </a>
                <a class="gh-sidebar-item">
                  <svg class="gh-sidebar-icon" viewBox="0 0 16 16" fill="currentColor"><path d="M2.75 2.5h10.5a.25.25 0 0 1 .25.25v10.5a.25.25 0 0 1-.25.25H2.75a.25.25 0 0 1-.25-.25V2.75a.25.25 0 0 1 .25-.25ZM13.25 1H2.75A1.75 1.75 0 0 0 1 2.75v10.5c0 .966.784 1.75 1.75 1.75h10.5A1.75 1.75 0 0 0 15 13.25V2.75A1.75 1.75 0 0 0 13.25 1Zm-1.5 5.5a.75.75 0 0 0 0-1.5h-7a.75.75 0 0 0 0 1.5h7Zm0 3a.75.75 0 0 0 0-1.5h-7a.75.75 0 0 0 0 1.5h7Z"/></svg>
                  账号
                </a>
                <a class="gh-sidebar-item">
                  <svg class="gh-sidebar-icon" viewBox="0 0 16 16" fill="currentColor"><path d="M8 0a8 8 0 1 1 0 16A8 8 0 0 1 8 0ZM5.78 8.75a9.64 9.64 0 0 0 1.9 3.48c.383-.477.78-1.102 1.1-1.947a20.2 20.2 0 0 0-1.1-1.947 9.64 9.64 0 0 0-1.9 3.48Z"/></svg>
                  外观
                </a>
              </div>
              <div class="gh-sidebar-section">
                <div class="gh-sidebar-section-title">访问</div>
                <a class="gh-sidebar-item">
                  <svg class="gh-sidebar-icon" viewBox="0 0 16 16" fill="currentColor"><path d="M2.75 2.5h10.5a.25.25 0 0 1 .25.25v10.5a.25.25 0 0 1-.25.25H2.75a.25.25 0 0 1-.25-.25V2.75a.25.25 0 0 1 .25-.25ZM13.25 1H2.75A1.75 1.75 0 0 0 1 2.75v10.5c0 .966.784 1.75 1.75 1.75h10.5A1.75 1.75 0 0 0 15 13.25V2.75A1.75 1.75 0 0 0 13.25 1Zm-1.5 5.5a.75.75 0 0 0 0-1.5h-7a.75.75 0 0 0 0 1.5h7Zm0 3a.75.75 0 0 0 0-1.5h-7a.75.75 0 0 0 0 1.5h7Z"/></svg>
                  电子邮件
                </a>
                <a class="gh-sidebar-item">
                  <svg class="gh-sidebar-icon" viewBox="0 0 16 16" fill="currentColor"><path d="M3.5 3.5c0-1.38 1.12-2.5 2.5-2.5h4c1.38 0 2.5 1.12 2.5 2.5v9c0 1.38-1.12 2.5-2.5 2.5h-4a2.5 2.5 0 0 1-2.5-2.5v-9Zm2.5-.5a.5.5 0 0 0-.5.5v9a.5.5 0 0 0 .5.5h4a.5.5 0 0 0 .5-.5v-9a.5.5 0 0 0-.5-.5h-4Z"/></svg>
                  密码与认证
                </a>
              </div>
            </nav>
          </aside>
          <div style="flex: 1; padding: 24px; background-color: var(--bgColor-default, #fff);">
            <h3 style="font-size: 24px; font-weight: 600; margin-bottom: 16px;">Applications</h3>
            <p style="color: var(--fgColor-muted, #656d76);">这里显示主内容区域</p>
          </div>
        </div>
      </div>

      <div class="mb-6">
        <h2 class="h2-mktg mb-3">头像</h2>
        <div class="d-flex">
          <img class="avatar avatar-small mr-2" src="https://github.com/github.png" alt="@github" width="32" height="32" />
          <img class="avatar mr-2" src="https://github.com/github.png" alt="@github" width="40" height="40" />
          <img class="avatar avatar-large mr-2" src="https://github.com/github.png" alt="@github" width="60" height="60" />
        </div>
      </div>

      <div class="mb-6">
        <h2 class="h2-mktg mb-3">分页</h2>
        <nav class="paginate" aria-label="Pagination">
          <a class="paginate-next" href="#" aria-label="Next Page">Next</a>
        </nav>
      </div>
    </div>
  </main>
</template>

<style>
/* 强制显示标题样式 */
h2.h2-mktg {
  display: block !important;
  font-size: 1.5rem !important;
  font-weight: 600 !important;
  margin-bottom: 0.75rem !important;
  visibility: visible !important;
  opacity: 1 !important;
}

h3.h3-mktg {
  display: block !important;
  font-size: 1.25rem !important;
  font-weight: 600 !important;
  margin-bottom: 0.5rem !important;
  visibility: visible !important;
  opacity: 1 !important;
}
</style>
