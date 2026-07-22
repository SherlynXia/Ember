<template>
  <div class="heatmap-wrap">
    <div class="month-label-row">
      <span
        v-for="month in fullDisplayMonthList"
        :key="month"
        class="month-label"
      >
        {{ month }}月
      </span>
    </div>
    <div class="heatmap-grid">
      <div
        v-for="cell in gridCellList"
        :key="cell.cellIndex"
        class="grid-cell"
        :style="{
          backgroundColor: cell.record ? cell.record.productivity_color : globalConfig.emptyCellColor
        }"
        @click="$emit('cell-click', cell)"
      >
        <div v-if="cell.record" class="cell-tooltip">
          {{ cell.record.date }} | {{ cell.record.mood_emoji }} {{ cell.record.comment || '无备注' }}
        </div>
      </div>
    </div>
    <div v-if="hasOverflowRecord" class="overflow-tip">
      记录超过{{ maxCellNum }}天，仅展示最新{{ maxCellNum }}条，可后端分页扩展
    </div>
  </div>
</template>

<script setup lang="ts">
import type { GridCell, GlobalThemeConfig } from '../../types/rating'

defineProps<{
  gridCellList: GridCell[]
  fullDisplayMonthList: number[]
  globalConfig: GlobalThemeConfig
  hasOverflowRecord: boolean
  maxCellNum: number
}>()
defineEmits(['cell-click'])
</script>

<style scoped>
.heatmap-wrap {
  margin-bottom: 40px;
  overflow-x: auto;
  padding-bottom: 8px;
}
.month-label-row {
  display: flex;
  justify-content: space-between;
  padding: 0 4px 8px 4px;
  min-width: max-content;
}
.month-label {
  font-size: 13px;
  color: v-bind('globalConfig.textSubColor');
  white-space: nowrap;
}
.heatmap-grid {
  display: grid;
  grid-template-columns: repeat(60, 1fr);
  gap: 2px;
  min-width: max-content;
}
.grid-cell {
  aspect-ratio: 1 / 1;
  border-radius: 2px;
  position: relative;
  cursor: pointer;
  transition: scale 0.15s ease;
}
.grid-cell:hover {
  transform: scale(1.2);
  z-index: 2;
}
.grid-cell:hover .cell-tooltip {
  display: block;
}
.cell-tooltip {
  display: none;
  position: absolute;
  bottom: 120%;
  left: 0;
  right: 0;
  transform: none;
  text-align: center;
  background: v-bind('globalConfig.tooltipBgColor');
  color: v-bind('globalConfig.tooltipTextColor');
  font-size: 12px;
  padding: 4px 8px;
  border-radius: 4px;
  white-space: nowrap;
  z-index: 99;
  max-width: 220px;
  overflow: hidden;
  text-overflow: ellipsis;
  box-shadow: 0 2px 8px rgba(0,0,0,0.4);
}
.overflow-tip {
  margin-top: 8px;
  font-size: 13px;
  color: #fbbf24;
}
</style>