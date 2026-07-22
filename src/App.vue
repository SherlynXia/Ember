<template>
  <div class="rating-container dark-theme">
    <!-- 顶部头部信息 -->
    <div class="header-wrap">
      <div class="title-area">
        <h2 class="main-title">我的每日评分记录</h2>
        <div class="stat-text">
          评分记录已陪伴你 {{ totalRecordDays }} 天，共 {{ totalRecordCount }} 条记录，最近连续 {{ continueDays }} 天打卡
        </div>
        <!-- 加载中提示 -->
        <div v-if="loading" class="loading-tip">⏳ 正在加载数据...</div>
        <!-- 接口加载失败提示 -->
        <div v-if="loadError" class="error-tip">
          ⚠️ 数据加载失败，请重试
          <button @click="fetchAllData" class="retry-btn">重试</button>
        </div>
        <!-- 空数据占位 -->
        <div v-if="!loading && !loadError && ratingList.length === 0" class="empty-tip">
          暂无评分记录，快去创建第一条每日记录吧
        </div>
      </div>
      <button 
        class="config-btn"
        :style="primaryBtnStyle"
        @click="openConfigPanel"
      >
        配色配置面板
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

    <!-- 每日评分 Tab 热力图 -->
    <div v-if="activeTab === 'rating'" class="heatmap-wrap">
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
          @click="handleCellClick(cell)"
        >
          <div v-if="cell.record" class="cell-tooltip">
            {{ cell.record.date }} | {{ cell.record.mood_emoji }} {{ cell.record.comment || '无备注' }}
          </div>
        </div>
      </div>
      <!-- 数据超出网格提示 -->
      <div v-if="hasOverflowRecord" class="overflow-tip">
        记录超过{{ TOTAL_CELL }}天，仅展示最新{{ TOTAL_CELL }}条，可后端分页扩展
      </div>
    </div>

    <!-- 成长度 Tab 预留区域 -->
    <div v-else class="growth-wrap">
      <p>成长度数据可视化区域，可接入图表展示 achievement_score 趋势</p>
    </div>

    <!-- 底部说明模块 -->
    <div class="bottom-desc-card">
      <h3 class="card-title">每日评分自动记录</h3>
      <p class="card-text">
        Tauri本地IPC通信，无需开启网络端口；配色、色块色全部由Rust后端下发，前端无硬编码色值
      </p>
      <button 
        class="open-config-btn"
        :style="primaryBtnStyle"
        @click="openConfigPanel"
      >
        打开配色管理
      </button>
      <!-- 右下角装饰点阵 -->
      <div class="decor-dot-group" :style="dotGridStyle">
        <div 
          v-for="dotIndex in dotTotalList" 
          :key="dotIndex"
          class="decor-dot"
          :style="{ backgroundColor: globalConfig.themePrimaryColor }"
        ></div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
// Tauri 核心调用方法，替代 fetch http 请求
import { invoke } from '@tauri-apps/api/core'

// ====================== 类型（与Rust完全对齐） ======================
export interface DailyRating {
  date: string
  status: number
  comment: string | null
  mood_emoji: string
  productivity_color: string
  work_hours: number
  achievement_score: number
  mit_completion_rate: number
  task_count_delta: number
  has_overdue: boolean
  has_milestone: boolean
}

export interface GlobalThemeConfig {
  themePrimaryColor: string
  emptyCellColor: string
  tabActiveBg: string
  textMainColor: string
  textSubColor: string
  cardBgColor: string
  pageBgColor: string
  themeBorderColor: string
  tooltipBgColor: string
  tooltipTextColor: string
}

interface GridCell {
  cellIndex: number
  date: string | null
  record: DailyRating | null
}

type RatingRes = { continueDays: number; list: DailyRating[] }


// ====================== 常量定义 ======================
const HEAT_ROW = 6
const HEAT_COL = 60
const TOTAL_CELL = HEAT_ROW * HEAT_COL
const DOT_COL = 12
const dotCount = DOT_COL * 10

// ====================== 响应式数据 ======================
const activeTab = ref<'rating' | 'growth'>('rating')
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
const continueDays = ref(0)
const loadError = ref(false)
const loading = ref(false)

// 复用按钮样式
const primaryBtnStyle = computed(() => ({
  backgroundColor: globalConfig.value.themePrimaryColor,
  color: globalConfig.value.textMainColor,
  border: 'none',
  transition: 'opacity 0.2s ease'
}))

// 点阵布局动态样式，解耦硬编码行列
const dotGridStyle = computed(() => ({
  gridTemplateColumns: `repeat(${DOT_COL}, 1fr)`
}))

// 完整覆盖热力图跨度月份（适配60列宽度）
const fullDisplayMonthList = computed(() => {
  if (!ratingList.value.length) return []
  const dates = ratingList.value.map(item => new Date(item.date))
  const minDate = new Date(Math.min(...dates.map(d => d.getTime())))
  const maxDate = new Date(Math.max(...dates.map(d => d.getTime())))
  const months: number[] = []
  let current = new Date(minDate)
  while (current <= maxDate) {
    months.push(current.getMonth() + 1)
    current.setMonth(current.getMonth() + 1)
  }
  return months
})

// 仅展示有数据的月份（去重+升序）
const displayMonthList = computed(() => {
  const existMonth = new Set<number>()
  ratingList.value.forEach(item => {
    const m = Number(item.date.split('-')[1])
    existMonth.add(m)
  })
  return Array.from(existMonth).sort((a, b) => a - b)
})

// 统计
const totalRecordDays = computed(() => new Set(ratingList.value.map(i => i.date)).size)
const totalRecordCount = computed(() => ratingList.value.length)

// 是否存在超出网格容量的数据
const hasOverflowRecord = computed(() => ratingList.value.length > TOTAL_CELL)

// 修复热力网格逻辑：最新日期放在网格末尾（右下角），符合行业热力图习惯
const gridCellList = computed<GridCell[]>(() => {
  // 1. 初始化空白格子
  const cells: GridCell[] = Array.from({ length: TOTAL_CELL }, (_, i) => ({
    cellIndex: i,
    date: null,
    record: null
  }))

  // 2. 记录按日期降序排序：新日期在前，旧日期在后
  const sortedRecords = [...ratingList.value].sort((a, b) => 
    new Date(b.date).getTime() - new Date(a.date).getTime()
  )

  // 3. 从网格末尾反向填充，最新数据在右下角
  const fillCount = Math.min(sortedRecords.length, TOTAL_CELL)
  for (let i = 0; i < fillCount; i++) {
    const cellIdx = TOTAL_CELL - 1 - i
    cells[cellIdx].date = sortedRecords[i].date
    cells[cellIdx].record = sortedRecords[i]
  }

  return cells
})

// 点阵循环数组
const dotTotalList = computed(() => Array.from({ length: dotCount }, (_, i) => i))

// ====================== Tauri IPC 请求（无端口、无http） ======================
// 获取主题配色
const fetchGlobalTheme = async () => {
  const data = await invoke<GlobalThemeConfig>("cmd_get_theme_config")
  console.log('✅ 主题配色加载成功：', data)
  globalConfig.value = data
}

// 获取评分记录
const fetchDailyRating = async () => {
  const res = await invoke<RatingRes>("cmd_get_rating_list")
  console.log('✅ 评分数据加载成功：', res)
  ratingList.value = res.list
  continueDays.value = res.continueDays
}

// 统一加载入口，并发调用两个tauri命令，加防抖锁
let fetchLock = false
const fetchAllData = async () => {
  if (fetchLock || loading.value) return
  fetchLock = true
  loading.value = true
  loadError.value = false
  try {
    // 拆分捕获，区分配色/评分接口错误
    await Promise.all([
      fetchGlobalTheme().catch(e => console.error('配色配置加载失败', e)),
      fetchDailyRating().catch(e => console.error('评分记录加载失败', e))
    ])
  } catch (err) {
    loadError.value = true
    console.error('❌ Tauri数据调用失败：', err)
  } finally {
    loading.value = false
    fetchLock = false
  }
}

// 格子点击
const handleCellClick = (cell: GridCell) => {
  if (!cell.record) return
  console.log('选中记录：', cell.record)
  // 后续可调用invoke打开编辑弹窗
}

// 打开配置面板（预留调用Rust弹窗/窗口命令）
const openConfigPanel = () => {
  console.log('打开配色配置面板')
  // invoke("cmd_open_config_window")
}

// 监听主题色变化，可同步更新其他逻辑
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
/* 移除硬编码背景色，完全由主题配置接管 */
.dark-theme {
  background-color: v-bind('globalConfig.pageBgColor');
  color: v-bind('globalConfig.textMainColor');
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
.overflow-tip {
  margin-top: 8px;
  font-size: 13px;
  color: #fbbf24;
}
.retry-btn {
  background: transparent;
  color: #60a5fa;
  border: 1px solid #60a5fa;
  border-radius: 4px;
  padding: 2px 8px;
  cursor: pointer;
}

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
.config-btn {
  border: none;
  border-radius: 999px;
  padding: 10px 18px;
  font-size: 14px;
  cursor: pointer;
  transition: opacity 0.2s;
}
.config-btn:hover, .open-config-btn:hover {
  opacity: 0.85;
}

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
.tab-desc {
  font-size: 14px;
  color: v-bind('globalConfig.textSubColor');
  margin-bottom: 20px;
}

.heatmap-wrap {
  margin-bottom: 40px;
  overflow-x: auto;
  padding-bottom: 8px;
}
.growth-wrap {
  margin-bottom: 40px;
  padding: 40px 0;
  color: v-bind('globalConfig.textSubColor');
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
  left: 50%;
  transform: translateX(-50%);
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
  /* 横向滚动时tooltip不超出容器 */
  left: 0;
  right: 0;
  transform: none;
  text-align: center;
}

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

/* 小屏幕适配 */
@media screen and (max-width: 768px) {
  .rating-container {
    padding: 16px;
  }
  .main-title {
    font-size: 22px;
  }
  .decor-dot-group {
    width: 160px;
    height: 160px;
    right: 20px;
    bottom: 10px;
  }
}
</style>