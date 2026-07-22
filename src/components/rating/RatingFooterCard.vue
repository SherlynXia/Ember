<template>
  <div class="bottom-desc-card">
    <h3 class="card-title">每日评分自动记录</h3>
    <p class="card-text">
      Tauri本地IPC通信，无需开启网络端口；配色、色块色全部由Rust后端下发，前端无硬编码色值
    </p>
    <button
      class="open-config-btn"
      :style="primaryBtnStyle"
      @click="$emit('open-config')"
    >
      打开配色管理
    </button>
    <div class="decor-dot-group" :style="dotGridStyle">
      <div
        v-for="dotIndex in dotTotalList"
        :key="dotIndex"
        class="decor-dot"
        :style="{ backgroundColor: globalConfig.themePrimaryColor }"
      ></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { GlobalThemeConfig } from '../../types/rating'
defineProps<{
  primaryBtnStyle: Record<string, string>
  dotGridStyle: Record<string, string>
  dotTotalList: number[]
  globalConfig: GlobalThemeConfig
}>()
defineEmits(['open-config'])
</script>

<style scoped>
.bottom-desc-card {
  background-color: v-bind('globalConfig.cardBgColor');
  border-radius: 12px;
  padding: 24px;
  position: relative;
  overflow: hidden;
}
.card-title {
  font-size: 20px;
  margin: 0 0 12px 0;
}
.card-text {
  color: v-bind('globalConfig.textSubColor');
  line-height: 1.6;
  max-width: 600px;
}
.open-config-btn {
  margin-top: 16px;
  border: none;
  border-radius: 6px;
  padding: 10px 20px;
  font-size: 14px;
  cursor: pointer;
  transition: opacity 0.2s;
}
.open-config-btn:hover {
  opacity: 0.85;
}
.decor-dot-group {
  position: absolute;
  right: 60px;
  bottom: 20px;
  width: 240px;
  height: 240px;
  gap: 3px;
  opacity: 0.7;
  pointer-events: none;
  display: grid;
}
.decor-dot {
  border-radius: 2px;
  aspect-ratio: 1/1;
}
@media screen and (max-width: 768px) {
  .decor-dot-group {
    width: 160px;
    height: 160px;
    right: 20px;
    bottom: 10px;
  }
}
</style>