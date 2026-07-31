<template>
  <div class="rating-container dark-theme">
    <RatingHeader
      :totalRecordDays="totalRecordDays"
      :totalRecordCount="totalRecordCount"
      :continueDays="continueDays"
      :loading="loading"
      :loadError="!!error"
      :ratingList="ratingList"
      :globalConfig="globalConfig"
      @reload="loadInitialData"
    />

    <!-- 记录今天按钮 -->
    <div class="action-bar">
      <button
        class="record-today-btn"
        :style="{
          backgroundColor: globalConfig.themePrimaryColor,
          color: '#fff'
        }"
        @click="openTodayInput"
      >
        <span class="btn-icon">✏️</span>
        <span>记录今天</span>
      </button>
    </div>

    <RatingTabBar
      v-model:activeTab="activeTab"
      :globalConfig="globalConfig"
    />

    <div class="tab-desc" :style="{ color: globalConfig.textSubColor }">
       
    </div>

    <RatingHeatmap
      v-if="activeTab === 'rating'"
      :gridCellList="gridCellList"
      :monthLabels="monthLabels"
      :currentYear="currentYear"
      :today="today"
      :globalConfig="globalConfig"
      @cell-click="handleCellClick"
      @cell-hover="handleCellHover"
    />

    <RatingGrowthPanel v-else :globalConfig="globalConfig" />

    <RatingFooterCard
      :dotGridStyle="dotGridStyle"
      :dotTotalList="dotTotalList"
      :globalConfig="globalConfig"
    />

    <!-- 每日评分输入弹窗 -->
    <RatingInputModal
      :visible="showInputModal"
      :date="selectedDate"
      :globalConfig="globalConfig"
      :initialData="initialDataForModal"
      :readonly="!isToday(selectedDate)"
      @close="showInputModal = false"
      @save="handleSaveRating"
    />

    <!-- 右侧详情面板 -->
    <DayDetailPanel
      :visible="showDetailPanel"
      :date="selectedDate"
      :rating="selectedRating"
      :tasks="selectedTasks"
      :loading="detailLoading"
      :globalConfig="globalConfig"
      @close="showDetailPanel = false"
      @edit="handleEditRating"
      @add="handleAddRating"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRatingApi } from '../composables/useRatingApi'
import { useRatingHeatmap } from '../composables/useRatingHeatmap'

import RatingHeader from '../components/rating/RatingHeader.vue'
import RatingTabBar from '../components/rating/RatingTabBar.vue'
import RatingHeatmap from '../components/rating/RatingHeatmap.vue'
import RatingGrowthPanel from '../components/rating/RatingGrowthPanel.vue'
import RatingFooterCard from '../components/rating/RatingFooterCard.vue'
import RatingInputModal from '../components/rating/RatingInputModal.vue'
import DayDetailPanel from '../components/rating/DayDetailPanel.vue'

import type { GridCell, DailyRating, SaveRatingForm, Task, GlobalThemeConfig } from '../types/rating'

const props = defineProps<{
  globalConfig: GlobalThemeConfig
}>()

const activeTab = ref<'rating' | 'growth'>('rating')

// 使用新的 API 组合式函数
const {
  ratingList,
  currentRating,
  currentTasks,
  globalConfig: apiGlobalConfig,
  loading,
  error,
  saveDailyRating,
  getDayDetail,
  loadInitialData,
  getToday
} = useRatingApi()

// 使用外部传入的 globalConfig，如果没有则使用 API 中的
const globalConfig = computed(() => props.globalConfig || apiGlobalConfig.value)

const {
  currentYear,
  totalRecordDays,
  totalRecordCount,
  gridCellList,
  monthLabels,
  dotTotalList,
  dotGridStyle
} = useRatingHeatmap(ratingList)

// 弹窗和面板状态
const showInputModal = ref(false)
const showDetailPanel = ref(false)
const selectedDate = ref(getToday())
// 当前日期（用于传递给子组件）
const today = getToday()
const selectedRating = ref<DailyRating | null>(null)
const selectedTasks = ref<Task[]>([])
const detailLoading = ref(false)

// 为 RatingInputModal 准备兼容的 initialData（将 comment: null -> undefined）
const initialDataForModal = computed<Partial<SaveRatingForm> | undefined>(() => {
  if (!selectedRating.value) return undefined
  const { comment, ...rest } = selectedRating.value as any
  return {
    ...(rest as any),
    comment: comment ?? undefined
  } as Partial<SaveRatingForm>
})

// 按钮样式
// 连续记录天数（简化计算）
const continueDays = computed(() => {
  if (ratingList.value.length === 0) return 0
  // TODO: 实现真正的连续记录计算
  return ratingList.value.length
})

// 判断是否为今天
const isToday = (dateStr: string): boolean => {
  return dateStr === getToday()
}

// 打开今天的输入弹窗
const openTodayInput = async () => {
  const today = getToday()
  selectedDate.value = today

  // 检查今天是否已有记录
  const todayRating = ratingList.value.find(r => r.date === today)
  if (todayRating) {
    selectedRating.value = todayRating
  } else {
    selectedRating.value = null
  }

  showInputModal.value = true
}

// 点击热力图格子
const handleCellClick = async (cell: GridCell) => {
  if (!cell.date) return

  selectedDate.value = cell.date
  selectedRating.value = cell.record

  // 加载详情
  detailLoading.value = true
  showDetailPanel.value = true
  selectedTasks.value = []

  // 如果没有记录，直接显示空面板
  if (!cell.record) {
    detailLoading.value = false
    return
  }

  try {
    const detail = await getDayDetail(cell.date)
    selectedTasks.value = detail.tasks
  } catch (err) {
    console.error('加载详情失败:', err)
    selectedTasks.value = []
  } finally {
    detailLoading.value = false
  }
}

// 悬停热力图格子
let hoverTimeout: ReturnType<typeof setTimeout> | null = null

const handleCellHover = (cell: GridCell) => {
  // TODO: 实现悬停提示浮窗
  // 可以在这里触发一个局部的 tooltip 事件
}

// 保存评分
const handleSaveRating = async (form: SaveRatingForm) => {
  try {
    await saveDailyRating(form)
    showInputModal.value = false

    // 如果详情面板打开，刷新详情
    if (showDetailPanel.value) {
      const detail = await getDayDetail(form.date)
      selectedRating.value = detail.rating
      selectedTasks.value = detail.tasks
    }
  } catch (err) {
    console.error('保存评分失败:', err)
    alert('保存失败，请重试')
  }
}

// 编辑评分
const handleEditRating = () => {
  showDetailPanel.value = false
  showInputModal.value = true
}

// 添加评分
const handleAddRating = () => {
  selectedRating.value = null
  showDetailPanel.value = false
  showInputModal.value = true
}

// 打开配置
// 初始化
onMounted(() => {
  loadInitialData()
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

.action-bar {
  display: flex;
  justify-content: flex-end;
  margin-bottom: 20px;
}

.record-today-btn {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 12px 24px;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
}

.record-today-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 16px rgba(59, 130, 246, 0.4);
}

.btn-icon {
  font-size: 16px;
}

.tab-desc {
  font-size: 14px;
  margin-bottom: 20px;
}

@media screen and (max-width: 768px) {
  .rating-container {
    padding: 16px;
  }

  .action-bar {
    justify-content: center;
  }

  .record-today-btn {
    width: 100%;
    justify-content: center;
  }
}
</style>