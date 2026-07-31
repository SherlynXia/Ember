<template>
  <Teleport to="body">
    <Transition name="panel">
      <div
        v-if="visible"
        class="detail-panel-overlay"
        @click.self="handleClose"
      >
        <div
          class="detail-panel"
          :style="{ backgroundColor: '#1e293b' }"
        >
          <!-- 头部 -->
          <div class="panel-header">
            <h3 :style="{ color: globalConfig.textMainColor }">
              {{ formatDisplayDate(date) }}
            </h3>
            <button
              class="close-btn"
              @click="handleClose"
              :style="{ color: globalConfig.textSubColor }"
            >
              ×
            </button>
          </div>

          <!-- 加载状态 -->
          <div v-if="loading" class="loading-state">
            <div class="spinner"></div>
            <span :style="{ color: globalConfig.textSubColor }">加载中...</span>
          </div>

          <!-- 内容区域 -->
          <div v-else class="panel-content">
            <!-- 有评分数据 -->
            <template v-if="rating">
              <!-- 心情和成就分 -->
              <div class="score-section">
                <div
                  class="mood-display"
                  :style="{ backgroundColor: globalConfig.pageBgColor }"
                >
                  <span class="mood-emoji">{{ rating.mood_emoji }}</span>
                  <span
                    class="mood-label"
                    :style="{ color: globalConfig.textSubColor }"
                  >
                    {{ getMoodLabel(rating.mood_emoji) }}
                  </span>
                </div>
                <div
                  class="achievement-score"
                  :style="{ backgroundColor: rating.productivity_color }"
                >
                  <span class="score-value">{{ rating.achievement_score }}</span>
                  <span class="score-label">成就分</span>
                </div>
              </div>

              <!-- 工作状态 -->
              <div
                class="info-section"
                :style="{ backgroundColor: globalConfig.pageBgColor }"
              >
                <div class="info-row">
                  <span class="info-label" :style="{ color: globalConfig.textSubColor }">
                    工作状态
                  </span>
                  <span
                    class="info-value work-type-badge"
                    :style="{
                      backgroundColor: getWorkTypeColor(rating.work_type) + '30',
                      color: getWorkTypeColor(rating.work_type),
                      borderColor: getWorkTypeColor(rating.work_type)
                    }"
                  >
                    {{ getWorkTypeLabel(rating.work_type) }}
                  </span>
                </div>
                <div class="info-row">
                  <span class="info-label" :style="{ color: globalConfig.textSubColor }">
                    工作时长
                  </span>
                  <span class="info-value" :style="{ color: globalConfig.textMainColor }">
                    {{ rating.work_hours }} 小时
                  </span>
                </div>
                <div class="info-row">
                  <span class="info-label" :style="{ color: globalConfig.textSubColor }">
                    能量-价值评分
                  </span>
                  <span class="info-value" :style="{ color: globalConfig.textMainColor }">
                    {{ rating.energy_score }} / {{ rating.value_score }}
                    <span
                      v-if="isGoldenZone(rating.energy_score, rating.value_score)"
                      class="golden-tag"
                    >
                      黄金区
                    </span>
                  </span>
                </div>
              </div>

              <!-- 评语 -->
              <div
                v-if="rating.comment"
                class="comment-section"
                :style="{ backgroundColor: globalConfig.pageBgColor }"
              >
                <div class="comment-label" :style="{ color: globalConfig.textSubColor }">
                  今日评语
                </div>
                <div class="comment-content" :style="{ color: globalConfig.textMainColor }">
                  {{ rating.comment }}
                </div>
              </div>

              <!-- 编辑按钮（仅当天可编辑） -->
              <button
                v-if="isToday(date)"
                class="edit-btn"
                :style="{
                  backgroundColor: globalConfig.themePrimaryColor,
                  color: '#fff'
                }"
                @click="handleEdit"
              >
                编辑评分
              </button>
              <div
                v-else
                class="readonly-notice"
                :style="{ color: globalConfig.textSubColor }"
              >
                往期记录不可修改
              </div>
            </template>

            <!-- 无评分数据 -->
            <template v-else>
              <div class="empty-state">
                <div class="empty-icon">📝</div>
                <div class="empty-text" :style="{ color: globalConfig.textSubColor }">
                  这一天还没有记录
                </div>
                <button
                  v-if="isToday(date)"
                  class="add-btn"
                  :style="{
                    backgroundColor: globalConfig.themePrimaryColor,
                    color: '#fff'
                  }"
                  @click="handleAdd"
                >
                  添加评分
                </button>
                <div
                  v-else
                  class="readonly-notice"
                  :style="{ color: globalConfig.textSubColor }"
                >
                  往期日期无法补录
                </div>
              </div>
            </template>

            <!-- 任务列表 -->
            <div class="tasks-section">
              <h4 :style="{ color: globalConfig.textMainColor }">
                当天任务 ({{ tasks.length }})
              </h4>

              <div v-if="tasks.length > 0" class="task-list">
                <div
                  v-for="task in tasks"
                  :key="task.id"
                  class="task-item"
                  :style="{ backgroundColor: globalConfig.pageBgColor }"
                >
                  <div class="task-header">
                    <span
                      class="task-status"
                      :class="{ completed: task.is_completed }"
                    >
                      {{ task.is_completed ? '✓' : '○' }}
                    </span>
                    <span
                      class="task-title"
                      :style="{ color: globalConfig.textMainColor }"
                      :class="{ 'task-completed': task.is_completed }"
                    >
                      {{ task.title }}
                    </span>
                    <span
                      v-if="task.is_important"
                      class="mit-badge"
                      :style="{ backgroundColor: globalConfig.themePrimaryColor + '30' }"
                    >
                      MIT
                    </span>
                  </div>

                  <div
                    v-if="task.is_completed"
                    class="task-meta"
                    :style="{ color: globalConfig.textSubColor }"
                  >
                    <span v-if="task.actual_time">
                      ⏱️ {{ task.actual_time }}h
                    </span>
                    <span v-if="task.actual_satisfaction">
                      ⭐ {{ task.actual_satisfaction }}/5
                    </span>
                    <span v-if="task.emoji_mood">
                      {{ task.emoji_mood }}
                    </span>
                  </div>

                  <div
                    v-if="task.comment"
                    class="task-comment"
                    :style="{ color: globalConfig.textSubColor }"
                  >
                    {{ task.comment }}
                  </div>
                </div>
              </div>

              <div v-else class="no-tasks" :style="{ color: globalConfig.textSubColor }">
                暂无指定任务
              </div>
            </div>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import type { DailyRating, Task, GlobalThemeConfig } from '../../types/rating'

const props = defineProps<{
  visible: boolean
  date: string
  rating: DailyRating | null
  tasks: Task[]
  loading: boolean
  globalConfig: GlobalThemeConfig
}>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'edit'): void
  (e: 'add'): void
}>()

// 心情标签映射
const moodLabels: Record<string, string> = {
  '😊': '爽/有成就感',
  '😐': '一般/平淡',
  '😫': '累/没进展',
  '🥳': '超常发挥'
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

const getMoodLabel = (emoji: string): string => {
  return moodLabels[emoji] || '未知'
}

const getWorkTypeLabel = (type: string): string => {
  return workTypeLabels[type] || type
}

const getWorkTypeColor = (type: string): string => {
  return workTypeColors[type] || '#888888'
}

const isGoldenZone = (energy: number, value: number): boolean => {
  return value >= 4 && energy <= 2
}

// 判断是否为今天
const isToday = (dateStr: string): boolean => {
  const today = new Date().toISOString().split('T')[0]
  return dateStr === today
}

const formatDisplayDate = (dateStr: string): string => {
  const date = new Date(dateStr)
  const year = date.getFullYear()
  const month = date.getMonth() + 1
  const day = date.getDate()
  const weekDays = ['日', '一', '二', '三', '四', '五', '六']
  const weekDay = weekDays[date.getDay()]
  return `${year}年${month}月${day}日 周${weekDay}`
}

const handleClose = () => {
  emit('close')
}

const handleEdit = () => {
  emit('edit')
}

const handleAdd = () => {
  emit('add')
}
</script>

<style scoped>
.detail-panel-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.3);
  z-index: 1000;
  display: flex;
  justify-content: flex-end;
}

.detail-panel {
  width: 100%;
  max-width: 400px;
  height: 100%;
  display: flex;
  flex-direction: column;
  box-shadow: -4px 0 20px rgba(0, 0, 0, 0.3);
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.panel-header h3 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
}

.close-btn {
  background: none;
  border: none;
  font-size: 24px;
  cursor: pointer;
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 6px;
  transition: background-color 0.2s;
}

.close-btn:hover {
  background-color: rgba(255, 255, 255, 0.1);
}

.panel-content {
  flex: 1;
  overflow-y: auto;
  padding: 24px;
}

/* 加载状态 */
.loading-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 60px 20px;
  gap: 16px;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 3px solid rgba(255, 255, 255, 0.1);
  border-top-color: #3b82f6;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

/* 分数展示区 */
.score-section {
  display: flex;
  gap: 16px;
  margin-bottom: 20px;
}

.mood-display {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 20px;
  border-radius: 12px;
  background-color: rgba(255, 255, 255, 0.08);
}

.mood-emoji {
  font-size: 48px;
  margin-bottom: 8px;
}

.mood-label {
  font-size: 14px;
}

.achievement-score {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 20px;
  border-radius: 12px;
  color: #fff;
  background-color: rgba(255, 255, 255, 0.08);
}

.score-value {
  font-size: 36px;
  font-weight: 700;
  line-height: 1;
}

.score-label {
  font-size: 12px;
  margin-top: 4px;
  opacity: 0.9;
}

/* 信息区域 */
.info-section {
  padding: 16px;
  border-radius: 10px;
  margin-bottom: 16px;
  background-color: rgba(255, 255, 255, 0.06);
}

.info-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 0;
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
}

.info-row:last-child {
  border-bottom: none;
}

.info-label {
  font-size: 14px;
}

.info-value {
  font-size: 14px;
  font-weight: 500;
}

.work-type-badge {
  padding: 4px 12px;
  border-radius: 20px;
  border: 1px solid;
  font-size: 13px;
}

.golden-tag {
  margin-left: 8px;
  background: linear-gradient(135deg, #fbbf24, #f59e0b);
  color: #000;
  padding: 2px 8px;
  border-radius: 10px;
  font-size: 11px;
  font-weight: 600;
}

/* 评语区域 */
.comment-section {
  padding: 16px;
  border-radius: 10px;
  margin-bottom: 16px;
  background-color: rgba(255, 255, 255, 0.06);
}

.comment-label {
  font-size: 12px;
  margin-bottom: 8px;
}

.comment-content {
  font-size: 14px;
  line-height: 1.6;
  white-space: pre-wrap;
}

/* 编辑按钮 */
.edit-btn {
  width: 100%;
  padding: 12px;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: opacity 0.2s;
  margin-bottom: 24px;
}

.edit-btn:hover {
  opacity: 0.9;
}

.readonly-notice {
  text-align: center;
  padding: 12px;
  font-size: 13px;
  margin-bottom: 24px;
  background-color: rgba(255, 255, 255, 0.1);
  border-radius: 8px;
}

/* 空状态 */
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 40px 20px;
  text-align: center;
}

.empty-icon {
  font-size: 48px;
  margin-bottom: 12px;
}

.empty-text {
  font-size: 14px;
  margin-bottom: 20px;
}

.add-btn {
  padding: 10px 24px;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: opacity 0.2s;
}

.add-btn:hover {
  opacity: 0.9;
}

/* 任务区域 */
.tasks-section {
  border-top: 1px solid rgba(255, 255, 255, 0.1);
  padding-top: 20px;
}

.tasks-section h4 {
  margin: 0 0 16px 0;
  font-size: 16px;
  font-weight: 600;
}

.task-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.task-item {
  padding: 14px;
  border-radius: 8px;
}

.task-header {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 8px;
}

.task-status {
  width: 20px;
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  font-size: 12px;
  background-color: rgba(255, 255, 255, 0.1);
  color: #888;
}

.task-status.completed {
  background-color: #22c55e;
  color: #fff;
}

.task-title {
  flex: 1;
  font-size: 14px;
  font-weight: 500;
}

.task-completed {
  text-decoration: line-through;
  opacity: 0.6;
}

.mit-badge {
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 11px;
  font-weight: 600;
  color: #3b82f6;
}

.task-meta {
  display: flex;
  gap: 16px;
  font-size: 12px;
  margin-left: 30px;
}

.task-comment {
  font-size: 12px;
  margin-left: 30px;
  margin-top: 8px;
  padding-top: 8px;
  border-top: 1px solid rgba(255, 255, 255, 0.05);
}

.no-tasks {
  text-align: center;
  padding: 30px;
  font-size: 14px;
}

/* 动画 */
.panel-enter-active,
.panel-leave-active {
  transition: opacity 0.3s ease;
}

.panel-enter-from,
.panel-leave-to {
  opacity: 0;
}

.panel-enter-active .detail-panel,
.panel-leave-active .detail-panel {
  transition: transform 0.3s ease;
}

.panel-enter-from .detail-panel {
  transform: translateX(100%);
}

.panel-leave-to .detail-panel {
  transform: translateX(100%);
}

@media screen and (max-width: 480px) {
  .detail-panel {
    max-width: 100%;
  }
}
</style>