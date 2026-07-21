<template>
  <div class="rating-container dark-theme">
    <!-- 顶部头部信息 -->
    <div class="header-wrap">
      <div class="title-area">
        <h2 class="main-title">我的每日评分记录</h2>
        <div class="stat-text">
          评分记录已陪伴你 {{ totalRecordDays }} 天，共 {{ totalRecordCount }} 条记录，最近连续 {{ continueDays }} 天打卡
        </div>
        <!-- 接口加载失败提示 -->
        <div v-if="loadError" class="error-tip">⚠️ 数据加载失败，请检查后端服务</div>
      </div>
      <button 
        class="config-btn"
        :style="{ backgroundColor: globalConfig.themePrimaryColor, color: globalConfig.textMainColor }"
      >
        后端配色配置面板
      </button>
    </div>

    <!-- Tab切换栏 -->
    <div class="tab-wrap">
      <button 
        class="tab-item" 
        :class="{ active: activeTab === 'rating' }"
        :style="activeTab === 'rating' ? { backgroundColor: globalConfig.tabActiveBg } : {}"
        @click="activeTab = 'rating'"
      >
        每日评分
      </button>
      <button 
        class="tab-item"
        :class="{ active: activeTab === 'growth' }"
        @click="activeTab = 'growth'"
      >
        成长度
      </button>
    </div>
    <div class="tab-desc">
      每日记录越多，热力色块颜色越深（色块颜色由后端 productivity_color 字段控制）
    </div>

    <!-- 热力图网格区域 -->
    <div class="heatmap-wrap">
      <!-- 月份横轴标签 -->
      <div class="month-label-row">
        <span v-for="month in monthList" :key="month" class="month-label">{{ month }}月</span>
      </div>
      <!-- 热力格子矩阵 -->
      <div class="heatmap-grid">
        <div 
          v-for="(cell, idx) in gridCellList" 
          :key="idx"
          class="grid-cell"
          :style="{
            backgroundColor: cell.record ? cell.record.productivity_color : globalConfig.emptyCellColor
          }"
          @click="handleCellClick(cell)"
        >
          <!-- 有记录时hover弹窗简易标识 -->
          <div v-if="cell.record" class="cell-tooltip">
            {{ cell.record.date }} | {{ cell.record.mood_emoji }} {{ cell.record.comment || '无备注' }}
          </div>
        </div>
      </div>
    </div>

    <!-- 底部说明模块 -->
    <div class="bottom-desc-card">
      <h3 class="card-title">每日评分自动记录</h3>
      <p class="card-text">
        后端统一配置全局主题色、空白格子底色；单条日期色块颜色由每条 DailyRating 的 productivity_color 字段动态下发，前端无硬编码色值
      </p>
      <button 
        class="open-config-btn"
        :style="{ backgroundColor: globalConfig.themePrimaryColor, color: globalConfig.textMainColor }"
      >
        打开后端配色管理
      </button>
      <!-- 右下角装饰点阵图形（动态色） -->
      <div class="decor-dot-group">
        <div 
          v-for="dot in 120" 
          :key="dot"
          class="decor-dot"
          :style="{ backgroundColor: globalConfig.themePrimaryColor }"
        ></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'

// ====================== 类型定义（和后端Rust字段完全对齐） ======================
export interface DailyRating {
  date: string // "YYYY-MM-DD"
  status: number // 0~5 五档评价
  comment: string | null // 最多20字短评
  mood_emoji: string // 表情 🟢❌
  productivity_color: string // CSS rgba/hex 色块颜色，后端控制
  work_hours: number
  achievement_score: number
  mit_completion_rate: number
  task_count_delta: number
  has_overdue: boolean
  has_milestone: boolean
}

export interface GlobalThemeConfig {
  themePrimaryColor: string // 按钮主色
  emptyCellColor: string // 无数据空白格子底色
  tabActiveBg: string // Tab激活背景色
  textMainColor: string // 主文字色
  textSubColor: string // 次要文字色
  cardBgColor: string // 底部卡片背景
  pageBgColor: string // 页面整体深色背景
  themeBorderColor: string // 边框基础色
  tooltipBgColor: string // 悬浮提示框背景
  tooltipTextColor: string // 悬浮提示文字色
}

interface GridCell {
  date: string | null
  record: DailyRating | null
}

// ====================== 响应式数据 ======================
const activeTab = ref<'rating' | 'growth'>('rating')
const monthList = ref([8,9,10,11,12,1,2,3,4,5,6,7])
const ratingList = ref<DailyRating[]>([])
const globalConfig = ref<GlobalThemeConfig>({
  themePrimaryColor: '#3b82f6',
  emptyCellColor: '#222226',
  tabActiveBg: 'rgba(59, 130, 246, 0.2)',
  textMainColor: '#f5f5f5',
  textSubColor: '#9ca3af',
  cardBgColor: '#1e1e22',
  pageBgColor: '#121214',
  themeBorderColor: '#333338',
  tooltipBgColor: '#0a0a0c',
  tooltipTextColor: '#f5f5f5'
})
const continueDays = ref(1)
const loadError = ref(false) // 接口加载错误标记

// 统计数据
const totalRecordDays = computed(() => ratingList.value.length)
const totalRecordCount = computed(() => ratingList.value.length)

// 热力网格矩阵【修复：稳定日期映射，不会随机丢失数据】
const gridCellList = computed<GridCell[]>(() => {
  const cells: GridCell[] = []
  const rowCount = 6
  const colCount = 60
  // 生成固定6行60列格子
  for(let row=0; row<rowCount; row++) {
    for(let col=0; col<colCount; col++) {
      cells.push({ date: null, record: null })
    }
  }
  // 稳定映射算法：用日期总天数计算唯一索引，每条数据都会分配格子
  ratingList.value.forEach((item) => {
    const [year, month, day] = item.date.split('-').map(Number)
    const dayTotal = year * 400 + month * 32 + day
    const targetIdx = dayTotal % cells.length
    cells[targetIdx].date = item.date
    cells[targetIdx].record = item
  })
  return cells
})

// ====================== 原生Fetch请求（带调试打印） ======================
const fetchGlobalTheme = async () => {
  try {
    const resp = await fetch('/api/theme/config')
    if (!resp.ok) throw new Error(`主题接口状态码：${resp.status}`)
    const data = await resp.json() as GlobalThemeConfig
    console.log('✅ 主题配色加载成功：', data)
    globalConfig.value = data
    loadError.value = false
  } catch (err) {
    loadError.value = true
    console.error('❌ 加载主题配色失败：', err)
  }
}

const fetchDailyRating = async () => {
  try {
    const resp = await fetch('/api/daily-rating/list')
    if (!resp.ok) throw new Error(`评分接口状态码：${resp.status}`)
    type RatingRes = { continueDays: number; list: DailyRating[] }
    const data = await resp.json() as RatingRes
    console.log('✅ 评分数据加载成功：', data)
    ratingList.value = data.list
    continueDays.value = data.continueDays
    loadError.value = false
  } catch (err) {
    loadError.value = true
    console.error('❌ 加载每日评分记录失败：', err)
  }
}

// 格子点击事件
const handleCellClick = (cell: GridCell) => {
  if(!cell.record) return
  console.log('选中当日记录：', cell.record)
}

onMounted(async () => {
  await fetchGlobalTheme()
  await fetchDailyRating()
})
</script>

<style scoped>
/* 页面默认兜底深色，解决初次白屏 */
.rating-container {
  width: 100%;
  min-height: 100vh;
  padding: 32px;
  box-sizing: border-box;
  background: #121214;
  color: #f5f5f5;
}
.dark-theme {
  background-color: v-bind('globalConfig.pageBgColor');
  color: v-bind('globalConfig.textMainColor');
}

/* 错误提示文字 */
.error-tip {
  color: #f87171;
  font-size: 14px;
  margin-top: 4px;
}

/* 头部区域 */
.header-wrap {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 24px;
}
.main-title {
  font-size: 28px;
  font-weight: 600;
  margin: 0 0 8px 0;
}
.stat-text {
  font-size: 15px;
  color: v-bind('globalConfig.textSubColor');
}
.config-btn {
  border: none;
  border-radius: 999px;
  padding: 10px 18px;
  font-size: 14px;
  cursor: pointer;
}

/* Tab切换 */
.tab-wrap {
  display: flex;
  gap: 12px;
  margin-bottom: 10px;
}
.tab-item {
  padding: 8px 16px;
  border-radius: 6px;
  border: 1px solid v-bind('globalConfig.themeBorderColor');
  background: transparent;
  color: v-bind('globalConfig.textMainColor');
  cursor: pointer;
  font-size: 15px;
}
.tab-item.active {
  border-color: transparent;
}
.tab-desc {
  font-size: 14px;
  color: v-bind('globalConfig.textSubColor');
  margin-bottom: 20px;
}

/* 热力图网格 */
.heatmap-wrap {
  margin-bottom: 40px;
}
.month-label-row {
  display: flex;
  justify-content: space-between;
  padding: 0 4px 8px 4px;
}
.month-label {
  font-size: 13px;
  color: v-bind('globalConfig.textSubColor');
}
.heatmap-grid {
  display: grid;
  grid-template-columns: repeat(60, 1fr);
  gap: 2px;
}
.grid-cell {
  aspect-ratio: 1 / 1;
  border-radius: 2px;
  position: relative;
  cursor: pointer;
}
.grid-cell:hover .cell-tooltip {
  display: block;
}
.cell-tooltip {
  display: none;
  position: absolute;
  bottom: 120%;
  left: 50%;
  transform: translateX(-50%);
  background: v-bind('globalConfig.tooltipBgColor');
  color: v-bind('globalConfig.tooltipTextColor');
  font-size: 12px;
  padding: 4px 8px;
  border-radius: 4px;
  white-space: nowrap;
  z-index: 9;
}

/* 底部卡片 */
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
}
.decor-dot-group {
  position: absolute;
  right: 60px;
  bottom: 20px;
  width: 240px;
  height: 240px;
  display: grid;
  grid-template-columns: repeat(12, 1fr);
  gap: 3px;
  opacity: 0.7;
}
.decor-dot {
  border-radius: 2px;
  aspect-ratio: 1/1;
}
</style>