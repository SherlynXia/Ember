<template>
  <div class="growth-wrap">
    <p>成长度数据可视化区域，可接入图表展示 achievement_score 趋势</p>
  </div>
</template>

<script setup lang="ts">
import type { GlobalThemeConfig } from '../../types/rating'

defineProps<{
  globalConfig: GlobalThemeConfig
}>()
// 后续图表逻辑写这里
</script>

<style scoped>
.growth-wrap {
  margin-bottom: 40px;
  padding: 40px 0;
  color: v-bind('globalConfig.textSubColor');
}
</style>