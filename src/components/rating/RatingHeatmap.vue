<template>
  <div class="heatmap-wrap">
    <!-- 月份标签行 -->
    <div class="month-label-row">
      <div
        v-for="label in monthLabels"
        :key="label.month"
        class="month-label"
        :style="{ left: `${label.weekIndex * (CELL_SIZE + CELL_GAP)}px` }"
      >
        {{ label.month }}月
      </div>
    </div>

    <!-- 热力图主体 -->
    <div class="heatmap-body">
      <!-- 星期标签列 -->
      <div class="weekday-labels">
        <div class="weekday-label">一</div>
        <div class="weekday-label"></div> <!-- 周二，通常不显示 -->
        <div class="weekday-label">三</div>
        <div class="weekday-label"></div> <!-- 周四，通常不显示 -->
        <div class="weekday-label">五</div>
        <div class="weekday-label"></div> <!-- 周六，通常不显示 -->
        <div class="weekday-label">日</div>
      </div>

      <!-- 热力图网格 -->
      <div class="heatmap-grid" :style="gridStyle">
        <template v-for="cell in gridCellList" :key="cell.cellIndex">
          <!-- 学年范围内的格子 -->
          <div
            v-if="cell.isInYearRange"
            class="grid-cell"
            :class="{
              'has-record': cell.record,
              'is-future': !cell.isPastOrToday,
              'is-today-empty': cell.date === today && !cell.record
            }"
            :style="{
              backgroundColor: cell.record ? cell.record.productivity_color : globalConfig.emptyCellColor,
              width: `${CELL_SIZE}px`,
              height: `${CELL_SIZE}px`
            }"
            @click="cell.isPastOrToday && $emit('cell-click', cell)"
            @mouseenter="cell.isPastOrToday && cell.record && showTooltip($event, cell)"
            @mouseleave="hideTooltip"
          >
          </div>
          <!-- 学年范围外的占位（完全不可见） -->
          <div
            v-else
            class="grid-cell-placeholder"
            :style="{
              width: `${CELL_SIZE}px`,
              height: `${CELL_SIZE}px`
            }"
          >
          </div>
        </template>
      </div>
    </div>

    <!-- 学年信息 -->
    <div class="year-info" :style="{ color: globalConfig.textSubColor }">
      {{ currentYear }}年7月 - {{ currentYear + 1 }}年6月
    </div>

    <!-- 全局悬浮提示 -->
    <Teleport to="body">
      <Transition name="tooltip">
        <div
          v-if="tooltipVisible && tooltipCell"
          class="global-tooltip"
          :style="{
            left: tooltipX + 'px',
            top: tooltipY + 'px',
            backgroundColor: globalConfig.tooltipBgColor,
            color: globalConfig.tooltipTextColor,
            borderColor: globalConfig.themeBorderColor
          }"
        >
          <div class="tooltip-date">{{ tooltipCell.record?.date }}</div>
          <div class="tooltip-mood">{{ tooltipCell.record?.mood_emoji }} {{ getMoodLabel(tooltipCell.record?.mood_emoji) }}</div>
          <div class="tooltip-work-type" :style="{ color: getWorkTypeColor(tooltipCell.record?.work_type) }">
            {{ getWorkTypeLabel(tooltipCell.record?.work_type) }}
          </div>
          <div class="tooltip-scores">
            <span>精力: {{ tooltipCell.record?.energy_score }}</span>
            <span>价值: {{ tooltipCell.record?.value_score }}</span>
            <span>时长: {{ tooltipCell.record?.work_hours }}h</span>
          </div>
          <div v-if="tooltipCell.record?.comment" class="tooltip-comment">
            {{ tooltipCell.record.comment }}
          </div>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import type { GridCell, GlobalThemeConfig, MonthLabel } from '../../types/rating'

const props = defineProps<{
  gridCellList: GridCell[]
  monthLabels: MonthLabel[]
  currentYear: number
  today: string
  globalConfig: GlobalThemeConfig
}>()

defineEmits(['cell-click', 'cell-hover'])

// 格子尺寸配置
const CELL_SIZE = 12
const CELL_GAP = 3

// 网格样式
const gridStyle = computed(() => ({
  display: 'grid',
  gridTemplateColumns: `repeat(53, ${CELL_SIZE}px)`,
  gridTemplateRows: `repeat(7, ${CELL_SIZE}px)`,
  gap: `${CELL_GAP}px`,
  gridAutoFlow: 'column' // 按列填充，实现周列布局
}))

// 心情标签映射
const moodLabels: Record<string, string> = {
  '😊': '爽',
  '😐': '一般',
  '😫': '累',
  '🥳': '超常'
}

// 工作状态标签映射
const workTypeLabels: Record<string, string> = {
  green: '高产日',
  blue: '学习日',
  orange: '低效日',
  gray: '休息日'
}

// 工作状态颜色映射
const workTypeColors: Record<string, string> = {
  green: '#22c55e',
  blue: '#3b82f6',
  orange: '#fb923c',
  gray: '#888888'
}

const getMoodLabel = (emoji: string | undefined): string => {
  if (!emoji) return ''
  return moodLabels[emoji] || ''
}

const getWorkTypeLabel = (type: string | undefined): string => {
  if (!type) return ''
  return workTypeLabels[type] || type
}

const getWorkTypeColor = (type: string | undefined): string => {
  if (!type) return '#888888'
  return workTypeColors[type] || '#888888'
}

// Tooltip 状态
const tooltipVisible = ref(false)
const tooltipCell = ref<GridCell | null>(null)
const tooltipX = ref(0)
const tooltipY = ref(0)

// 显示提示
const showTooltip = (event: MouseEvent, cell: GridCell) => {
  if (!cell.record) return

  tooltipCell.value = cell
  tooltipVisible.value = true

  // 计算提示框位置（在鼠标上方）
  const rect = (event.target as HTMLElement).getBoundingClientRect()
  tooltipX.value = rect.left + rect.width / 2 - 100  // 居中
  tooltipY.value = rect.top - 140  // 在格子上方

  // 确保不超出屏幕
  if (tooltipX.value < 10) tooltipX.value = 10
  if (tooltipX.value > window.innerWidth - 210) tooltipX.value = window.innerWidth - 210
  if (tooltipY.value < 10) tooltipY.value = rect.bottom + 10
}

// 隐藏提示
const hideTooltip = () => {
  tooltipVisible.value = false
  tooltipCell.value = null
}
</script>

<style scoped>
.heatmap-wrap {
  margin-bottom: 40px;
  overflow-x: auto;
  padding-bottom: 8px;
}

/* 月份标签行 */
.month-label-row {
  position: relative;
  height: 20px;
  margin-bottom: 8px;
  margin-left: 30px; /* 为星期标签留出空间 */
}

.month-label {
  position: absolute;
  font-size: 12px;
  color: v-bind('globalConfig.textSubColor');
  white-space: nowrap;
}

/* 热力图主体 */
.heatmap-body {
  display: flex;
  align-items: flex-start;
}

/* 星期标签列 */
.weekday-labels {
  display: flex;
  flex-direction: column;
  gap: 3px;
  margin-right: 8px;
  padding-top: 0;
}

.weekday-label {
  height: 12px;
  line-height: 12px;
  font-size: 10px;
  color: v-bind('globalConfig.textSubColor');
  text-align: right;
  width: 22px;
}

/* 热力图网格 */
.heatmap-grid {
  /* 样式由计算属性动态生成 */
}

/* 正常格子 */
.grid-cell {
  border-radius: 2px;
  cursor: pointer;
  transition: all 0.15s ease;
}

.grid-cell:hover {
  transform: scale(1.3);
  z-index: 2;
  box-shadow: 0 0 0 2px rgba(255, 255, 255, 0.5);
}

/* 未来日期（明天及以后）- 显示但不可交互 */
.grid-cell.is-future {
  cursor: default;
  opacity: 0.3;
}

.grid-cell.is-future:hover {
  transform: none;
  box-shadow: none;
}

/* 今天但未填写 - 显示为半透明，但可点击填写 */
.grid-cell.is-today-empty {
  opacity: 0.3;
}

/* 占位格子（学年范围外）- 完全透明不可见 */
.grid-cell-placeholder {
  background: transparent;
  pointer-events: none;
}

/* 学年信息 */
.year-info {
  margin-top: 12px;
  font-size: 12px;
  text-align: center;
}

/* 全局悬浮提示 */
.global-tooltip {
  position: fixed;
  width: 200px;
  padding: 12px;
  border-radius: 8px;
  border: 1px solid;
  font-size: 13px;
  z-index: 9999;
  pointer-events: none;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5);
}

.tooltip-date {
  font-weight: 600;
  margin-bottom: 8px;
  padding-bottom: 8px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.tooltip-mood {
  font-size: 16px;
  margin-bottom: 4px;
}

.tooltip-work-type {
  font-weight: 500;
  margin-bottom: 8px;
}

.tooltip-scores {
  display: flex;
  gap: 12px;
  font-size: 12px;
  opacity: 0.8;
  margin-bottom: 8px;
}

.tooltip-comment {
  font-size: 12px;
  font-style: italic;
  opacity: 0.9;
  padding-top: 8px;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
  white-space: pre-wrap;
  word-break: break-word;
  max-height: 60px;
  overflow: hidden;
}

/* 动画 */
.tooltip-enter-active,
.tooltip-leave-active {
  transition: opacity 0.2s ease, transform 0.2s ease;
}

.tooltip-enter-from,
.tooltip-leave-to {
  opacity: 0;
  transform: translateY(10px);
}
</style>