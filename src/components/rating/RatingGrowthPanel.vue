<template>
  <div class="growth-wrap">
    <div class="growth-header">
      <h3 class="growth-title">📈 成就趋势</h3>
      <div class="growth-summary" v-if="trendPoints.length > 0">
        <span class="summary-item">
          当前 <strong>{{ currentScore }}</strong> 分
        </span>
        <span class="summary-divider">|</span>
        <span class="summary-item" :class="trendClass">
          趋势 {{ trendDirection }}
        </span>
      </div>
    </div>

    <!-- 空状态 -->
    <div v-if="trendPoints.length === 0 && !chartLoading" class="empty-state">
      <span class="empty-icon">📊</span>
      <p>还没有足够的评分数据来生成趋势图</p>
      <p class="empty-hint">开始每日记录，积累数据后会在这里展示成就趋势</p>
    </div>

    <!-- 加载状态 -->
    <div v-if="chartLoading" class="loading-state">
      <span class="loading-spinner"></span>
      <p>加载趋势数据...</p>
    </div>

    <!-- SVG 趋势图 -->
    <div v-if="trendPoints.length > 0 && !chartLoading" class="chart-container">
      <svg
        :width="svgWidth"
        :height="svgHeight"
        :viewBox="`0 0 ${svgWidth} ${svgHeight}`"
        class="trend-chart"
        @mousemove="handleChartHover"
        @mouseleave="handleChartLeave"
      >
        <!-- 背景网格线 -->
        <line
          v-for="(y, i) in yGridLines"
          :key="'grid-' + i"
          :x1="padding.left"
          :y1="y"
          :x2="svgWidth - padding.right"
          :y2="y"
          :stroke="globalConfig.themeBorderColor"
          stroke-width="0.5"
          stroke-dasharray="4,4"
        />

        <!-- Y轴标签 -->
        <text
          v-for="(y, i) in yGridLines"
          :key="'ylabel-' + i"
          :x="padding.left - 8"
          :y="y + 4"
          :fill="globalConfig.textSubColor"
          font-size="10"
          text-anchor="end"
        >{{ yLabels[i] }}</text>

        <!-- X轴日期标签 -->
        <text
          v-for="(label, i) in xDateLabels"
          :key="'xlabel-' + i"
          :x="label.x"
          :y="svgHeight - padding.bottom + 16"
          :fill="globalConfig.textSubColor"
          font-size="9"
          text-anchor="middle"
        >{{ label.text }}</text>

        <!-- 区域填充 -->
        <path
          :d="areaPath"
          :fill="`url(#trendGradient)`"
          opacity="0.3"
        />

        <!-- 折线 -->
        <path
          :d="linePath"
          fill="none"
          :stroke="globalConfig.themePrimaryColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        />

        <!-- 悬停竖线 -->
        <line
          v-if="hoverIndex !== null && chartPoints[hoverIndex]"
          :x1="chartPoints[hoverIndex].x"
          :y1="padding.top"
          :x2="chartPoints[hoverIndex].x"
          :y2="svgHeight - padding.bottom"
          :stroke="globalConfig.themePrimaryColor"
          stroke-width="1"
          stroke-dasharray="3,3"
          opacity="0.5"
        />
        <!-- 悬停圆点 -->
        <circle
          v-if="hoverIndex !== null && chartPoints[hoverIndex]"
          :cx="chartPoints[hoverIndex].x"
          :cy="chartPoints[hoverIndex].y"
          r="4"
          :fill="globalConfig.themePrimaryColor"
          stroke="#fff"
          stroke-width="2"
        />

        <!-- 渐变定义 -->
        <defs>
          <linearGradient id="trendGradient" x1="0" y1="0" x2="0" y2="1">
            <stop offset="0%" :stop-color="globalConfig.themePrimaryColor" stop-opacity="0.4" />
            <stop offset="100%" :stop-color="globalConfig.themePrimaryColor" stop-opacity="0.02" />
          </linearGradient>
        </defs>
      </svg>

      <!-- 悬停提示 -->
      <div
        v-if="hoverTooltip && hoverTooltip.visible"
        class="chart-tooltip"
        :style="{
          left: hoverTooltip.x + 'px',
          top: hoverTooltip.y + 'px',
          backgroundColor: globalConfig.tooltipBgColor,
          color: globalConfig.tooltipTextColor,
          borderColor: globalConfig.themeBorderColor
        }"
      >
        <div class="tooltip-date">{{ hoverTooltip.date }}</div>
        <div class="tooltip-score">
          成就分: <strong>{{ hoverTooltip.score }}</strong>
        </div>
        <div class="tooltip-type">{{ hoverTooltip.workTypeLabel }}</div>
      </div>
    </div>

    <!-- 时间范围选择 -->
    <div v-if="trendPoints.length > 0" class="range-selector">
      <button
        v-for="r in ranges"
        :key="r.days"
        class="range-btn"
        :class="{ active: selectedRange === r.days }"
        :style="selectedRange === r.days ? {
          backgroundColor: globalConfig.themePrimaryColor,
          color: '#fff'
        } : {
          backgroundColor: 'transparent',
          color: globalConfig.textSubColor,
          borderColor: globalConfig.themeBorderColor
        }"
        @click="changeRange(r.days)"
      >{{ r.label }}</button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRatingApi } from '../../composables/useRatingApi'
import type { GlobalThemeConfig } from '../../types/rating'

defineProps<{
  globalConfig: GlobalThemeConfig
}>()

const { trendData, getAchievementTrend } = useRatingApi()

// 图表状态
const chartLoading = ref(false)
const selectedRange = ref(30)
const hoverIndex = ref<number | null>(null)
const hoverTooltip = ref<{
  visible: boolean
  x: number
  y: number
  date: string
  score: number
  workTypeLabel: string
} | null>(null)

const ranges = [
  { label: '7天', days: 7 },
  { label: '14天', days: 14 },
  { label: '30天', days: 30 },
  { label: '60天', days: 60 },
]

// SVG 画布尺寸
const svgWidth = 720
const svgHeight = 280
const padding = { top: 24, right: 16, bottom: 32, left: 42 }

// 从 API 获取的趋势数据
const trendPoints = computed(() => trendData.value)

// 趋势摘要
const currentScore = computed(() => {
  if (trendPoints.value.length === 0) return 0
  return trendPoints.value[trendPoints.value.length - 1].achievement_score
})

const trendDirection = computed(() => {
  if (trendPoints.value.length < 2) return '—'
  const recent = trendPoints.value.slice(-7)
  const first = recent[0].achievement_score
  const last = recent[recent.length - 1].achievement_score
  if (last > first + 3) return '↑ 上升'
  if (last < first - 3) return '↓ 下降'
  return '→ 平稳'
})

const trendClass = computed(() => {
  if (trendDirection.value.includes('上升')) return 'trend-up'
  if (trendDirection.value.includes('下降')) return 'trend-down'
  return 'trend-stable'
})

// Y轴
const yGridLines = computed(() => {
  const count = 5
  const top = padding.top
  const bottom = svgHeight - padding.bottom
  return Array.from({ length: count + 1 }, (_, i) =>
    top + ((bottom - top) / count) * i
  )
})

const yLabels = computed(() =>
  Array.from({ length: 6 }, (_, i) => Math.round(100 - (100 / 5) * i))
)

// 数据点坐标
const chartPoints = computed(() => {
  const data = trendPoints.value
  if (data.length === 0) return []

  const chartW = svgWidth - padding.left - padding.right
  const chartH = svgHeight - padding.top - padding.bottom

  return data.map((pt, i) => ({
    x: padding.left + (data.length > 1 ? (chartW / (data.length - 1)) * i : chartW / 2),
    y: padding.top + chartH * (1 - pt.achievement_score / 100),
    ...pt
  }))
})

// SVG 折线路径（贝塞尔平滑）
const linePath = computed(() => {
  const pts = chartPoints.value
  if (pts.length === 0) return ''
  if (pts.length === 1) return `M${pts[0].x},${pts[0].y}`

  let d = `M${pts[0].x},${pts[0].y}`
  for (let i = 1; i < pts.length; i++) {
    const prev = pts[i - 1]
    const curr = pts[i]
    const cpx1 = prev.x + (curr.x - prev.x) / 3
    const cpx2 = prev.x + ((curr.x - prev.x) / 3) * 2
    d += ` C${cpx1},${prev.y} ${cpx2},${curr.y} ${curr.x},${curr.y}`
  }
  return d
})

// 区域填充路径
const areaPath = computed(() => {
  const pts = chartPoints.value
  if (pts.length === 0) return ''
  const bottomY = svgHeight - padding.bottom
  const line = linePath.value
  if (pts.length === 1) return `${line} L${pts[0].x},${bottomY} Z`
  return `${line} L${pts[pts.length - 1].x},${bottomY} L${pts[0].x},${bottomY} Z`
})

// X轴日期标签
const xDateLabels = computed(() => {
  const data = trendPoints.value
  if (data.length === 0) return []

  const labels: { x: number; text: string }[] = []
  const interval = Math.max(1, Math.floor(data.length / 8))

  for (let i = 0; i < data.length; i += interval) {
    const pt = chartPoints.value[i]
    if (pt) {
      const parts = data[i].date.split('-')
      labels.push({ x: pt.x, text: `${parts[1]}/${parts[2]}` })
    }
  }
  return labels
})

// 工作类型映射
const workTypeLabels: Record<string, string> = {
  green: '🟢 高产日',
  blue: '🔵 学习日',
  orange: '🟠 低效日',
  gray: '⚪ 休息日',
}

// 图表悬停交互
const handleChartHover = (e: MouseEvent) => {
  const svgEl = e.currentTarget as SVGSVGElement
  const rect = svgEl.getBoundingClientRect()
  const scaleX = svgWidth / rect.width
  const mouseX = (e.clientX - rect.left) * scaleX

  const pts = chartPoints.value
  if (pts.length === 0) return

  let nearestIdx = 0
  let minDist = Infinity
  for (let i = 0; i < pts.length; i++) {
    const dist = Math.abs(pts[i].x - mouseX)
    if (dist < minDist) {
      minDist = dist
      nearestIdx = i
    }
  }

  const threshold = svgWidth / (pts.length * 0.5)
  if (minDist > threshold) {
    hoverIndex.value = null
    hoverTooltip.value = null
    return
  }

  hoverIndex.value = nearestIdx
  const pt = pts[nearestIdx]
  hoverTooltip.value = {
    visible: true,
    x: e.offsetX + 12,
    y: e.offsetY - 60,
    date: pt.date,
    score: pt.achievement_score,
    workTypeLabel: workTypeLabels[pt.work_type] || pt.work_type,
  }
}

const handleChartLeave = () => {
  hoverIndex.value = null
  hoverTooltip.value = null
}

// 切换时间范围
const changeRange = async (days: number) => {
  selectedRange.value = days
  chartLoading.value = true
  try {
    await getAchievementTrend(days)
  } finally {
    chartLoading.value = false
  }
}

// 初始化加载
onMounted(async () => {
  chartLoading.value = true
  try {
    await getAchievementTrend(selectedRange.value)
  } finally {
    chartLoading.value = false
  }
})
</script>

<style scoped>
.growth-wrap {
  margin-bottom: 40px;
  padding: 24px 0;
}

.growth-header {
  display: flex;
  align-items: baseline;
  justify-content: space-between;
  margin-bottom: 20px;
  padding: 0 8px;
}

.growth-title {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: v-bind('globalConfig.textMainColor');
}

.growth-summary {
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 13px;
  color: v-bind('globalConfig.textSubColor');
}

.summary-item strong {
  color: v-bind('globalConfig.textMainColor');
  font-size: 16px;
}

.summary-divider {
  opacity: 0.3;
}

.trend-up { color: #22c55e !important; }
.trend-down { color: #ef4444 !important; }
.trend-stable { color: v-bind('globalConfig.textSubColor'); }

/* 空状态 & 加载 */
.empty-state,
.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  text-align: center;
}

.empty-icon {
  font-size: 48px;
  margin-bottom: 16px;
  opacity: 0.6;
}

.empty-state p {
  margin: 4px 0;
  font-size: 14px;
  color: v-bind('globalConfig.textSubColor');
}

.empty-hint {
  font-size: 12px !important;
  opacity: 0.6;
}

.loading-spinner {
  width: 32px;
  height: 32px;
  border: 2px solid v-bind('globalConfig.themeBorderColor');
  border-top-color: v-bind('globalConfig.themePrimaryColor');
  border-radius: 50%;
  animation: spin 0.8s linear infinite;
  margin-bottom: 12px;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.loading-state p {
  font-size: 13px;
  color: v-bind('globalConfig.textSubColor');
}

/* 图表 */
.chart-container {
  position: relative;
  width: 100%;
  overflow: hidden;
  border-radius: 8px;
  background: v-bind('globalConfig.cardBgColor');
  border: 1px solid v-bind('globalConfig.themeBorderColor');
}

.trend-chart {
  display: block;
  width: 100%;
  height: auto;
}

/* 悬停提示 */
.chart-tooltip {
  position: absolute;
  pointer-events: none;
  z-index: 10;
  padding: 8px 12px;
  border-radius: 6px;
  border: 1px solid;
  font-size: 12px;
  line-height: 1.6;
  white-space: nowrap;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}

.tooltip-date { font-size: 11px; opacity: 0.7; }
.tooltip-score strong { font-size: 15px; color: v-bind('globalConfig.themePrimaryColor'); }
.tooltip-type { font-size: 11px; opacity: 0.8; }

/* 范围选择 */
.range-selector {
  display: flex;
  gap: 8px;
  margin-top: 16px;
  padding: 0 8px;
}

.range-btn {
  padding: 6px 16px;
  border-radius: 6px;
  border: 1px solid;
  font-size: 13px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.range-btn:hover { opacity: 0.85; }
.range-btn.active { font-weight: 600; }
</style>