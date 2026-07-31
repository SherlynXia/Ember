<template>
  <div class="app-container">
    <!-- 顶部导航 -->
    <nav class="app-nav" :style="{ backgroundColor: globalConfig.cardBgColor }">
      <div class="nav-brand">Ember</div>
      <div class="nav-tabs">
        <button
          :class="['nav-tab', { active: currentView === 'rating' }]"
          :style="currentView === 'rating' ? { color: globalConfig.themePrimaryColor } : { color: globalConfig.textSubColor }"
          @click="currentView = 'rating'"
        >
          📊 成长图
        </button>
        <button
          :class="['nav-tab', { active: currentView === 'tasks' }]"
          :style="currentView === 'tasks' ? { color: globalConfig.themePrimaryColor } : { color: globalConfig.textSubColor }"
          @click="currentView = 'tasks'"
        >
          ✅ 任务管理
        </button>
      </div>
    </nav>

    <!-- 页面内容 -->
    <main class="app-main">
      <RatingRecord
        v-if="currentView === 'rating'"
        :global-config="globalConfig"
      />
      <TaskBoard
        v-else-if="currentView === 'tasks'"
        :global-config="globalConfig"
      />
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { onMounted } from 'vue'
import RatingRecord from './views/RatingRecord.vue'
import TaskBoard from './views/TaskBoard.vue'
import { useUpdater } from './composables/useUpdater'
import type { GlobalThemeConfig } from './types/rating'

const { checkForUpdatesSilent } = useUpdater()

const currentView = ref<'rating' | 'tasks'>('rating')

// 全局主题配置
const globalConfig: GlobalThemeConfig = {
  themePrimaryColor: '#3b82f6',
  emptyCellColor: 'rgba(255, 255, 255, 0.05)',
  tabActiveBg: 'rgba(59, 130, 246, 0.15)',
  textMainColor: '#e5e7eb',
  textSubColor: '#9ca3af',
  cardBgColor: '#1e293b',
  pageBgColor: '#0f172a',
  themeBorderColor: 'rgba(255, 255, 255, 0.1)',
  tooltipBgColor: 'rgba(30, 41, 59, 0.95)',
  tooltipTextColor: '#e5e7eb'
}

// 启动时自动检查更新
onMounted(() => {
  setTimeout(() => {
    checkForUpdatesSilent()
  }, 3000)
})
</script>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  background-color: #0f172a;
  color: #e5e7eb;
}

.app-container {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.app-nav {
  display: flex;
  align-items: center;
  padding: 12px 24px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  gap: 32px;
}

.nav-brand {
  font-size: 20px;
  font-weight: 700;
  background: linear-gradient(135deg, #3b82f6, #8b5cf6);
  -webkit-background-clip: text;
  background-clip: text;
  -webkit-text-fill-color: transparent;
}

.nav-tabs {
  display: flex;
  gap: 8px;
}

.nav-tab {
  padding: 8px 16px;
  border: none;
  background: none;
  font-size: 14px;
  cursor: pointer;
  border-radius: 8px;
  transition: all 0.2s;
}

.nav-tab:hover {
  background: rgba(255, 255, 255, 0.05);
}

.nav-tab.active {
  background: rgba(59, 130, 246, 0.15);
  font-weight: 500;
}

.app-main {
  flex: 1;
  overflow: auto;
}
</style>