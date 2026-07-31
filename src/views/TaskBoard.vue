<template>
  <div class="task-board" :style="{ backgroundColor: globalConfig.pageBgColor }">
    <!-- 头部 -->
    <div class="board-header">
      <h2 :style="{ color: globalConfig.textMainColor }">任务管理</h2>
      <button
        class="add-task-btn"
        :style="{ backgroundColor: globalConfig.themePrimaryColor }"
        @click="showCreateModal = true"
      >
        + 新建任务
      </button>
    </div>

    <!-- 任务看板 -->
    <div class="board-content">
      <!-- 左侧：任务列表 -->
      <div class="task-panels">
        <!-- 待办池 -->
        <div class="task-panel">
          <div class="panel-header">
            <h3 :style="{ color: globalConfig.textSubColor }">
              📥 待安排
              <span class="count-badge">{{ unassignedTasks.length }}</span>
            </h3>
          </div>
          <div
            class="panel-body"
            @dragover.prevent
            @drop="handleDropToUnassigned"
          >
            <TaskCard
              v-for="task in unassignedTasks"
              :key="task.id"
              :task="task"
              :global-config="globalConfig"
              @edit="handleEdit"
              @delete="handleDelete"
              @assign="handleAssign"
              @split="handleSplit"
              @complete="handleComplete"
            />
            <div v-if="unassignedTasks.length === 0" class="empty-state">
              暂无待安排任务
            </div>
          </div>
        </div>

        <!-- 已安排 -->
        <div class="task-panel">
          <div class="panel-header">
            <h3 :style="{ color: globalConfig.textSubColor }">
              📅 已安排
              <span class="count-badge">{{ assignedTasks.length }}</span>
            </h3>
          </div>
          <div class="panel-body">
            <TaskCard
              v-for="task in assignedTasks"
              :key="task.id"
              :task="task"
              :global-config="globalConfig"
              @edit="handleEdit"
              @delete="handleDelete"
              @assign="handleAssign"
              @split="handleSplit"
              @complete="handleComplete"
            />
            <div v-if="assignedTasks.length === 0" class="empty-state">
              暂无已安排任务
            </div>
          </div>
        </div>

        <!-- 已完成 -->
        <div class="task-panel">
          <div class="panel-header">
            <h3 :style="{ color: globalConfig.textSubColor }">
              ✅ 已完成
              <span class="count-badge">{{ completedTasks.length }}</span>
            </h3>
          </div>
          <div class="panel-body">
            <TaskCard
              v-for="task in completedTasks"
              :key="task.id"
              :task="task"
              :global-config="globalConfig"
              @edit="handleEdit"
              @delete="handleDelete"
            />
            <div v-if="completedTasks.length === 0" class="empty-state">
              暂无已完成任务
            </div>
          </div>
        </div>
      </div>

      <!-- 右侧：日期分配区 -->
      <div class="date-assignment">
        <h3 :style="{ color: globalConfig.textSubColor }">
          📆 拖拽分配日期
        </h3>
        <div class="date-grid">
          <div
            v-for="dateInfo in upcomingDates"
            :key="dateInfo.date"
            class="date-cell"
            :class="{ 
              'is-today': dateInfo.isToday,
              'drag-over': dragOverDate === dateInfo.date 
            }"
            :style="dateCellStyle(dateInfo)"
            @dragover.prevent="dragOverDate = dateInfo.date"
            @dragleave="dragOverDate = null"
            @drop="handleDropToDate($event, dateInfo.date)"
            @click="showDateTasks(dateInfo.date)"
          >
            <div class="date-label">{{ dateInfo.label }}</div>
            <div class="date-weekday">{{ dateInfo.weekday }}</div>
            <div v-if="getTasksForDate(dateInfo.date).length > 0" class="task-indicators">
              <span
                v-for="task in getTasksForDate(dateInfo.date).slice(0, 3)"
                :key="task.id"
                class="task-dot"
                :class="{ 'is-mit': task.is_important }"
              />
              <span v-if="getTasksForDate(dateInfo.date).length > 3" class="more-tasks">
                +{{ getTasksForDate(dateInfo.date).length - 3 }}
              </span>
            </div>
          </div>
        </div>

        <!-- 选中日期的任务列表 -->
        <div v-if="selectedDate" class="selected-date-tasks">
          <h4 :style="{ color: globalConfig.textMainColor }">
            {{ selectedDate }} 的任务
          </h4>
          <TaskCard
            v-for="task in getTasksForDate(selectedDate)"
            :key="task.id"
            :task="task"
            :global-config="globalConfig"
            @edit="handleEdit"
            @delete="handleDelete"
            @assign="handleAssign"
          />
        </div>
      </div>
    </div>

    <!-- 创建/编辑任务弹窗 -->
    <TaskEditModal
      v-if="showCreateModal"
      :visible="showCreateModal"
      :global-config="globalConfig"
      @close="showCreateModal = false"
      @save="handleCreate"
    />

    <TaskEditModal
      v-if="editingTask"
      :visible="!!editingTask"
      :task="editingTask"
      :global-config="globalConfig"
      @close="editingTask = null"
      @save="handleUpdate"
    />

    <!-- 完成任务弹窗 -->
    <CompleteTaskModal
      v-if="completingTaskId"
      :visible="!!completingTaskId"
      :task-id="completingTaskId"
      :global-config="globalConfig"
      @close="completingTaskId = null"
      @complete="handleCompleteSubmit"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import type { Task, GlobalThemeConfig } from '../types/rating'
import {
  getAllTasks,
  createTask,
  updateTask,
  deleteTask,
  assignTaskToDate,
  createSubTask,
  completeTask
} from '../composables/useTaskApi'
import TaskCard from '../components/task/TaskCard.vue'
import TaskEditModal from '../components/task/TaskEditModal.vue'
import CompleteTaskModal from '../components/task/CompleteTaskModal.vue'

const props = defineProps<{
  globalConfig: GlobalThemeConfig
}>()

const tasks = ref<Task[]>([])
const showCreateModal = ref(false)
const editingTask = ref<Task | null>(null)
const completingTaskId = ref<number | null>(null)
const selectedDate = ref<string | null>(null)
const dragOverDate = ref<string | null>(null)

// 分类任务
const unassignedTasks = computed(() =>
  tasks.value.filter(t => !t.assigned_date && !t.is_completed && !t.parent_id)
)

const assignedTasks = computed(() =>
  tasks.value.filter(t => t.assigned_date && !t.is_completed && !t.parent_id)
)

const completedTasks = computed(() =>
  tasks.value.filter(t => t.is_completed && !t.parent_id)
)

// 生成未来14天的日期
const upcomingDates = computed(() => {
  const dates = []
  const today = new Date()
  const weekdays = ['日', '一', '二', '三', '四', '五', '六']

  for (let i = 0; i < 14; i++) {
    const date = new Date(today)
    date.setDate(today.getDate() + i)
    const dateStr = date.toISOString().split('T')[0]

    dates.push({
      date: dateStr,
      label: i === 0 ? '今天' : i === 1 ? '明天' : `${date.getMonth() + 1}/${date.getDate()}`,
      weekday: weekdays[date.getDay()],
      isToday: i === 0
    })
  }

  return dates
})

// 获取某天的任务
const getTasksForDate = (dateStr: string) => {
  return tasks.value.filter(t => t.assigned_date === dateStr && !t.is_completed)
}

// 日期格子样式
const dateCellStyle = (dateInfo: { isToday: boolean }) => {
  if (dateInfo.isToday) {
    return {
      borderColor: props.globalConfig.themePrimaryColor,
      backgroundColor: 'rgba(59, 130, 246, 0.1)'
    }
  }
  return {
    borderColor: 'rgba(255, 255, 255, 0.1)',
    backgroundColor: props.globalConfig.cardBgColor
  }
}

// 加载任务
const loadTasks = async () => {
  try {
    tasks.value = await getAllTasks()
  } catch (err) {
    console.error('加载任务失败:', err)
  }
}

// 创建任务
const handleCreate = async (taskData: Partial<Task>) => {
  try {
    await createTask(
      taskData.title!,
      taskData.estimated_time || 1,
      taskData.estimated_difficulty || 3,
      taskData.is_important || false
    )
    showCreateModal.value = false
    await loadTasks()
  } catch (err) {
    console.error('创建任务失败:', err)
    alert('创建任务失败: ' + err)
  }
}

// 更新任务
const handleUpdate = async (taskData: Partial<Task>) => {
  if (!editingTask.value) return
  try {
    await updateTask(editingTask.value.id, {
      title: taskData.title,
      estimatedTime: taskData.estimated_time,
      estimatedDifficulty: taskData.estimated_difficulty,
      isImportant: taskData.is_important,
      comment: taskData.comment
    })
    editingTask.value = null
    await loadTasks()
  } catch (err) {
    console.error('更新任务失败:', err)
    alert('更新任务失败: ' + err)
  }
}

// 删除任务
const handleDelete = async (taskId: number) => {
  if (!confirm('确定要删除这个任务吗？')) return
  try {
    await deleteTask(taskId)
    await loadTasks()
  } catch (err) {
    console.error('删除任务失败:', err)
    alert('删除任务失败: ' + err)
  }
}

// 分配日期
const handleAssign = async (payload: { taskId: number; dateStr: string | null }) => {
  try {
    await assignTaskToDate(payload.taskId, payload.dateStr)
    await loadTasks()
  } catch (err) {
    console.error('分配日期失败:', err)
    alert('分配日期失败: ' + err)
  }
}

// 拖拽到未分配区域
const handleDropToUnassigned = (e: DragEvent) => {
  dragOverDate.value = null
  const taskId = e.dataTransfer?.getData('taskId')
  if (taskId) {
    handleAssign({ taskId: parseInt(taskId), dateStr: null })
  }
}

// 拖拽到日期
const handleDropToDate = (e: DragEvent, dateStr: string) => {
  dragOverDate.value = null
  const taskId = e.dataTransfer?.getData('taskId')
  if (taskId) {
    handleAssign({ taskId: parseInt(taskId), dateStr })
  }
}

// 拆分任务
const handleSplit = async (payload: { parentId: number; title: string }) => {
  try {
    await createSubTask(payload.parentId, payload.title, 1, 3)
    await loadTasks()
  } catch (err) {
    console.error('创建子任务失败:', err)
    alert('创建子任务失败: ' + err)
  }
}

// 完成任务
const handleComplete = (taskId: number) => {
  completingTaskId.value = taskId
}

// 提交完成
const handleCompleteSubmit = async (data: {
  taskId: number
  actualTime: number
  actualSatisfaction: number
  emojiMood: string
  energyScore: number
  valueScore: number
  comment?: string
}) => {
  try {
    await completeTask(
      data.taskId,
      data.actualTime,
      data.actualSatisfaction,
      data.emojiMood,
      data.energyScore,
      data.valueScore,
      data.comment
    )
    completingTaskId.value = null
    await loadTasks()
  } catch (err) {
    console.error('完成任务失败:', err)
    alert('完成任务失败: ' + err)
  }
}

// 显示日期任务
const showDateTasks = (dateStr: string) => {
  selectedDate.value = dateStr
}

// 编辑任务
const handleEdit = (task: Task) => {
  editingTask.value = task
}

onMounted(() => {
  loadTasks()
})
</script>

<style scoped>
.task-board {
  min-height: 100vh;
  padding: 20px;
}

.board-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
}

.board-header h2 {
  font-size: 24px;
  font-weight: 600;
}

.add-task-btn {
  padding: 10px 20px;
  border: none;
  border-radius: 8px;
  color: white;
  font-size: 14px;
  cursor: pointer;
  transition: opacity 0.2s;
}

.add-task-btn:hover {
  opacity: 0.9;
}

.board-content {
  display: grid;
  grid-template-columns: 2fr 1fr;
  gap: 24px;
}

.task-panels {
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.task-panel {
  background: rgba(255, 255, 255, 0.03);
  border-radius: 12px;
  padding: 16px;
}

.panel-header {
  margin-bottom: 12px;
}

.panel-header h3 {
  font-size: 16px;
  font-weight: 600;
  display: flex;
  align-items: center;
  gap: 8px;
}

.count-badge {
  background: rgba(255, 255, 255, 0.1);
  padding: 2px 10px;
  border-radius: 12px;
  font-size: 12px;
}

.panel-body {
  min-height: 100px;
}

.empty-state {
  text-align: center;
  padding: 40px;
  opacity: 0.5;
  font-size: 14px;
}

.date-assignment {
  background: rgba(255, 255, 255, 0.03);
  border-radius: 12px;
  padding: 16px;
}

.date-assignment h3 {
  font-size: 16px;
  font-weight: 600;
  margin-bottom: 16px;
}

.date-grid {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 8px;
}

.date-cell {
  aspect-ratio: 1;
  border-radius: 8px;
  border: 2px solid;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: all 0.2s;
  position: relative;
  padding: 4px;
}

.date-cell:hover,
.date-cell.drag-over {
  transform: scale(1.05);
  border-color: #3b82f6 !important;
  background: rgba(59, 130, 246, 0.2) !important;
}

.date-label {
  font-size: 12px;
  font-weight: 600;
}

.date-weekday {
  font-size: 10px;
  opacity: 0.7;
}

.task-indicators {
  display: flex;
  gap: 2px;
  margin-top: 4px;
}

.task-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: #3b82f6;
}

.task-dot.is-mit {
  background: #fbbf24;
}

.more-tasks {
  font-size: 8px;
  opacity: 0.7;
}

.selected-date-tasks {
  margin-top: 20px;
  padding-top: 20px;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
}

.selected-date-tasks h4 {
  font-size: 14px;
  margin-bottom: 12px;
}
</style>