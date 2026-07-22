<template>
  <div class="header-wrap">
    <div class="title-area">
      <h2 class="main-title">我的每日评分记录</h2>
      <div class="stat-text">
        评分记录已陪伴你 {{ totalRecordDays }} 天，共 {{ totalRecordCount }} 条记录，最近连续 {{ continueDays }} 天打卡
      </div>
      <div v-if="loading" class="loading-tip">⏳ 正在加载数据...</div>
      <div v-if="loadError" class="error-tip">
        ⚠️ 数据加载失败，请重试
        <button @click="$emit('reload')" class="retry-btn">重试</button>
      </div>
      <div v-if="!loading && !loadError && ratingList.length === 0" class="empty-tip">
        暂无评分记录，快去创建第一条每日记录吧
      </div>
    </div>
    <button class="config-btn" :style="primaryBtnStyle" @click="$emit('open-config')">
      配色配置面板
    </button>
  </div>
</template>

<script setup lang="ts">
import type { DailyRating, GlobalThemeConfig } from '../../types/rating'
defineProps<{
  totalRecordDays: number
  totalRecordCount: number
  continueDays: number
  loading: boolean
  loadError: boolean
  ratingList: DailyRating[]
  primaryBtnStyle: Record<string, string>
  globalConfig: GlobalThemeConfig
}>()
defineEmits(['reload', 'open-config'])
</script>

<style scoped>
.header-wrap {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 24px;
  gap: 20px;
  flex-wrap: wrap;
}
.main-title {
  font-size: 28px;
  font-weight: 600;
  margin: 0 0 8px 0;
}
.stat-text {
  font-size: 15px;
  color: v-bind('globalConfig.textSubColor');
  line-height: 1.5;
}
.loading-tip {
  color: #60a5fa;
  font-size: 14px;
  margin-top: 4px;
}
.error-tip {
  color: #f87171;
  font-size: 14px;
  margin-top: 4px;
  display: flex;
  gap: 10px;
  align-items: center;
}
.empty-tip {
  color: v-bind('globalConfig.textSubColor');
  font-size: 14px;
  margin-top: 4px;
}
.retry-btn {
  background: transparent;
  color: #60a5fa;
  border: 1px solid #60a5fa;
  border-radius: 4px;
  padding: 2px 8px;
  cursor: pointer;
}
.config-btn {
  border: none;
  border-radius: 999px;
  padding: 10px 18px;
  font-size: 14px;
  cursor: pointer;
  transition: opacity 0.2s;
}
.config-btn:hover {
  opacity: 0.85;
}
@media screen and (max-width: 768px) {
  .main-title {
    font-size: 22px;
  }
}
</style>