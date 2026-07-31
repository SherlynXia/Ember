<template>
  <div
    class="task-card"
    :class="{
      'is-completed': task.is_completed,
      'is-important': task.is_important,
      'is-sub-task': task.parent_id,
      'is-dragging': isDragging
    }"
    :style="cardStyle"
    draggable="true"
    @dragstart="handleDragStart"
    @dragend="handleDragEnd"
  >
    <!-- 拖拽时的半透明覆盖层 -->
    <div v-if="isDragging" class="drag-overlay">
      <span>📅 拖拽到日期</span>
    </div>

    <!-- 任务头部 -->
    <div class="task-header">
      <div class="task-title-row">
        <span v-if="task.is_important" class="mit-badge">MIT</span>
        <span v-if="task.parent_id" class="subtask-indicator">└─</span>
        <span class="task-title">{{ task.title }}</span>
      </div>
      <div class="task-actions">
        <button class="action-btn" title="拆分任务" @click.stop="handleSplit">✂️</button>
        <button class="action-btn" title="编辑" @click.stop="$emit('edit', task)">✏️</button>
        <button class="action-btn" title="删除" @click.stop="$emit('delete', task.id)">🗑️</button>
      </div>
    </div>

    <!-- 任务描述（可编辑） -->
    <div
      v-if="task.comment"
      class="task-comment"
      :style="{ color: globalConfig.textSubColor }"
    >
      {{ task.comment }}
    </div>

    <!-- 任务信息 -->
    <div class="task-info">
      <span class="info-item" title="预估耗时">
        ⏱️ {{ task.estimated_time }}h
      </span>
      <span class="info-item" title="预估难度">
        📊 {{ '⭐'.repeat(task.estimated_difficulty) }}
      </span>
      <span v-if="task.assigned_date" class="info-item" title="分配日期">
        📅 {{ formatDate(task.assigned_date) }}
      </span>
      <span v-if="task.is_completed && task.actual_time" class="info-item" title="实际耗时">
        ✅ {{ task.actual_time }}h
      </span>
    </div>

    <!-- 完成状态详情 -->
    <div v-if="task.is_completed" class="completion-info">
      <span class="mood">{{ task.emoji_mood }}</span>
      <span class="satisfaction">满意度 {{ task.actual_satisfaction }}/5</span>
      <span v-if="task.energy_score && task.value_score" class="efficiency">
        效率 {{ (task.value_score / task.energy_score).toFixed(1) }}
      </span>
    </div>

    <!-- 子任务区域 -->
    <div v-if="task.sub_task_ids && task.sub_task_ids.length > 0" class="subtasks-section">
      <button
        class="subtask-toggle"
        @click.stop="toggleSubTasks"
      >
        <span>{{ isExpanded ? '▼' : '▶' }}</span>
        <span>子任务 ({{ task.sub_task_ids.length }})</span>
      </button>
      
      <!-- 子任务列表 -->
      <div v-if="isExpanded" class="subtask-list">
        <TaskCard
          v-for="subTask in subTasks"
          :key="subTask.id"
          :task="subTask"
          :global-config="globalConfig"
          @edit="$emit('edit', $event)"
          @delete="$emit('delete', $event)"
          @assign="$emit('assign', $event)"
          @split="$emit('split', $event)"
        />
      </div>
    </div>

    <!-- 快速操作栏 -->
    <div v-if="!task.is_completed" class="quick-actions">
      <button
        v-if="!task.assigned_date"
        class="quick-btn"
        @click.stop="$emit('assign', { taskId: task.id, dateStr: getToday() })"
      >
        📅 分配到今天
      </button>
      <button
        v-else
        class="quick-btn"
        @click.stop="$emit('assign', { taskId: task.id, dateStr: null })"
      >
        🔄 取消分配
      </button>
      <button
        class="quick-btn complete-btn"
        @click.stop="$emit('complete', task.id)"
      >
        ✓ 完成
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import type { Task, GlobalThemeConfig } from '../../types/rating'

const props = defineProps<{
  task: Task
  globalConfig: GlobalThemeConfig
  subTasks?: Task[]
  isExpanded?: boolean
}>()

const emit = defineEmits<{
  edit: [task: Task]
  delete: [taskId: number]
  assign: [payload: { taskId: number; dateStr: string | null }]
  split: [payload: { parentId: number; title: string }]
  complete: [taskId: number]
  toggleSubtasks: [taskId: number]
}>()

const isDragging = ref(false)
const showSubTasks = ref(false)

const isExpanded = computed(() => showSubTasks.value)

const cardStyle = computed(() => ({
  backgroundColor: props.task.is_completed
    ? 'rgba(255,255,255,0.03)'
    : props.globalConfig.cardBgColor,
  borderLeftColor: props.task.is_important
    ? props.globalConfig.themePrimaryColor
    : 'transparent',
  opacity: props.task.is_completed ? 0.6 : 1
}))

const handleDragStart = (e: DragEvent) => {
  if (props.task.is_completed) {
    e.preventDefault()
    return
  }
  isDragging.value = true
  e.dataTransfer?.setData('taskId', props.task.id.toString())
  e.dataTransfer!.effectAllowed = 'move'
}

const handleDragEnd = () => {
  isDragging.value = false
}

const handleSplit = () => {
  const title = prompt('请输入子任务标题：')
  if (title && title.trim()) {
    emit('split', { parentId: props.task.id, title: title.trim() })
  }
}

const toggleSubTasks = () => {
  showSubTasks.value = !showSubTasks.value
  emit('toggleSubtasks', props.task.id)
}

const formatDate = (dateStr: string) => {
  const date = new Date(dateStr)
  return `${date.getMonth() + 1}/${date.getDate()}`
}

const getToday = () => {
  return new Date().toISOString().split('T')[0]
}
</script>

<style scoped>
.task-card {
  padding: 14px;
  border-radius: 10px;
  border: 1px solid rgba(255, 255, 255, 0.05);
  border-left: 3px solid transparent;
  cursor: grab;
  transition: all 0.2s;
  position: relative;
  margin-bottom: 8px;
}

.task-card:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  border-color: rgba(255, 255, 255, 0.1);
}

.task-card.is-dragging {
  opacity: 0.5;
  cursor: grabbing;
}

.task-card.is-sub-task {
  margin-left: 24px;
  border-left-style: dashed;
}

.drag-overlay {
  position: absolute;
  inset: 0;
  background: rgba(59, 130, 246, 0.9);
  border-radius: 10px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: white;
  font-weight: 500;
  z-index: 10;
  pointer-events: none;
}

.task-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 8px;
}

.task-title-row {
  display: flex;
  align-items: center;
  gap: 6px;
  flex: 1;
  min-width: 0;
}

.mit-badge {
  background: linear-gradient(135deg, #fbbf24, #f59e0b);
  color: #000;
  font-size: 10px;
  font-weight: bold;
  padding: 2px 6px;
  border-radius: 4px;
  flex-shrink: 0;
}

.subtask-indicator {
  color: rgba(255, 255, 255, 0.4);
  font-size: 12px;
}

.task-title {
  font-size: 14px;
  font-weight: 500;
  color: inherit;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.task-actions {
  display: flex;
  gap: 4px;
  opacity: 0;
  transition: opacity 0.2s;
  flex-shrink: 0;
}

.task-card:hover .task-actions {
  opacity: 1;
}

.action-btn {
  background: rgba(255, 255, 255, 0.05);
  border: none;
  cursor: pointer;
  padding: 4px 6px;
  border-radius: 4px;
  font-size: 12px;
  transition: background 0.2s;
}

.action-btn:hover {
  background: rgba(255, 255, 255, 0.15);
}

.task-comment {
  font-size: 12px;
  margin-bottom: 8px;
  padding: 4px 8px;
  background: rgba(255, 255, 255, 0.03);
  border-radius: 4px;
  overflow: hidden;
  text-overflow: ellipsis;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
}

.task-info {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-bottom: 8px;
}

.info-item {
  font-size: 12px;
  opacity: 0.7;
  background: rgba(255, 255, 255, 0.05);
  padding: 2px 8px;
  border-radius: 4px;
}

.completion-info {
  display: flex;
  align-items: center;
  gap: 12px;
  padding-top: 8px;
  border-top: 1px solid rgba(255, 255, 255, 0.05);
  font-size: 12px;
}

.mood {
  font-size: 16px;
}

.satisfaction,
.efficiency {
  opacity: 0.7;
}

.subtasks-section {
  margin-top: 8px;
}

.subtask-toggle {
  width: 100%;
  padding: 6px;
  background: rgba(255, 255, 255, 0.03);
  border: 1px solid rgba(255, 255, 255, 0.05);
  border-radius: 6px;
  color: inherit;
  font-size: 12px;
  cursor: pointer;
  display: flex;
  align-items: center;
  gap: 6px;
}

.subtask-toggle:hover {
  background: rgba(255, 255, 255, 0.08);
}

.subtask-list {
  margin-top: 8px;
  padding-left: 8px;
  border-left: 2px solid rgba(255, 255, 255, 0.05);
}

.quick-actions {
  display: flex;
  gap: 8px;
  margin-top: 10px;
  padding-top: 10px;
  border-top: 1px solid rgba(255, 255, 255, 0.05);
}

.quick-btn {
  flex: 1;
  padding: 6px 10px;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 6px;
  color: inherit;
  font-size: 12px;
  cursor: pointer;
  transition: all 0.2s;
}

.quick-btn:hover {
  background: rgba(255, 255, 255, 0.1);
}

.complete-btn {
  background: rgba(34, 197, 94, 0.15);
  border-color: rgba(34, 197, 94, 0.3);
  color: #4ade80;
}

.complete-btn:hover {
  background: rgba(34, 197, 94, 0.25);
}
</style>