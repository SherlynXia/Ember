<template>
  <div class="tab-wrap">
    <button
      class="tab-item"
      :class="{ active: activeTab === 'rating' }"
      :style="activeTab === 'rating' ? { backgroundColor: globalConfig.tabActiveBg } : {}"
      @click="updateModel('rating')"
    >
      每日评分
    </button>
    <button
      class="tab-item"
      :class="{ active: activeTab === 'growth' }"
      @click="updateModel('growth')"
    >
      成长度
    </button>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { GlobalThemeConfig } from '../../types/rating'

type TabType = 'rating' | 'growth'

const props = defineProps<{
  activeTab: TabType
  globalConfig: GlobalThemeConfig
}>()

const emit = defineEmits<{
  'update:activeTab': [val: TabType]
}>()

const updateModel = (val: TabType) => {
  emit('update:activeTab', val)
}
</script>

<style scoped>
.tab-wrap {
  display: flex;
  gap: 12px;
  margin-bottom: 10px;
  flex-wrap: wrap;
}
.tab-item {
  padding: 8px 16px;
  border-radius: 6px;
  border: 1px solid v-bind('globalConfig.themeBorderColor');
  background: transparent;
  color: v-bind('globalConfig.textMainColor');
  cursor: pointer;
  font-size: 15px;
  transition: all 0.2s ease;
}
.tab-item.active {
  border-color: transparent;
}
</style>