<template>
  <div class="rating-container dark-theme">
      <RatingHeader
      :totalRecordDays="totalRecordDays"
      :totalRecordCount="totalRecordCount"
      :continueDays="continueDays"
      :loading="loading"
      :loadError="loadError"
      :ratingList="ratingList"
      :primaryBtnStyle="primaryBtnStyle"
      :globalConfig="globalConfig"
      @reload="fetchAllData"
      @open-config="handleOpenConfig"
    />
    <RatingTabBar
      v-model:activeTab="activeTab"
      :globalConfig="globalConfig"
    />

    <div class="tab-desc">
      每日记录越多，热力色块颜色越深（色块颜色由后端 productivity_color 字段控制）
    </div>

    <RatingHeatmap
      v-if="activeTab === 'rating'"
      :gridCellList="gridCellList"
      :fullDisplayMonthList="fullDisplayMonthList"
      :globalConfig="globalConfig"
      :hasOverflowRecord="hasOverflowRecord"
      :maxCellNum="TOTAL_CELL"
      @cell-click="handleCellClick"
    />

    <RatingGrowthPanel v-else :globalConfig="globalConfig" />

    
    <RatingFooterCard
      :primaryBtnStyle="primaryBtnStyle"
      :dotGridStyle="dotGridStyle"
      :dotTotalList="dotTotalList"
      :globalConfig="globalConfig"
      @open-config="handleOpenConfig"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useRatingApi } from '../composables/useRatingApi'
import { useRatingHeatmap, TOTAL_CELL } from '../composables/useRatingHeatmap'

import RatingHeader from '../components/rating/RatingHeader.vue'
import RatingTabBar from '../components/rating/RatingTabBar.vue'
import RatingHeatmap from '../components/rating/RatingHeatmap.vue'
import RatingGrowthPanel from '../components/rating/RatingGrowthPanel.vue'
import RatingFooterCard from '../components/rating/RatingFooterCard.vue'
import type { GridCell } from '../types/rating'

const activeTab = ref<'rating' | 'growth'>('rating')

const {
  ratingList,
  globalConfig,
  continueDays,
  loading,
  loadError,
  fetchAllData
} = useRatingApi()

const {
  totalRecordDays,
  totalRecordCount,
  hasOverflowRecord,
  fullDisplayMonthList,
  gridCellList,
  dotTotalList,
  dotGridStyle
} = useRatingHeatmap(ratingList)

const primaryBtnStyle = computed(() => ({
  backgroundColor: globalConfig.value.themePrimaryColor,
  color: globalConfig.value.textMainColor,
  border: 'none',
  transition: 'opacity 0.2s ease'
}))

const handleCellClick = (cell: GridCell) => {
  if (!cell.record) return
  console.log('选中记录：', cell.record)
}

const handleOpenConfig = () => {
  console.log('打开配色配置面板')
}

watch(globalConfig, () => {}, { deep: true })

onMounted(() => {
  fetchAllData()
})
</script>

<style scoped>
.rating-container {
  width: 100%;
  min-height: 100vh;
  padding: 32px;
  box-sizing: border-box;
}
.dark-theme {
  background-color: v-bind('globalConfig.pageBgColor');
  color: v-bind('globalConfig.textMainColor');
}
.tab-desc {
  font-size: 14px;
  color: v-bind('globalConfig.textSubColor');
  margin-bottom: 20px;
}

@media screen and (max-width: 768px) {
  .rating-container {
    padding: 16px;
  }
}
</style>