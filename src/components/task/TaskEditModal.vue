<template>
  <div class="modal-overlay" @click.self="$emit('close')">
    <div class="modal-content" :style="{ backgroundColor: globalConfig.cardBgColor }">
      <h3 :style="{ color: globalConfig.textMainColor }">
        {{ task ? '编辑任务' : '新建任务' }}
      </h3>

      <div class="form-group">
        <label :style="{ color: globalConfig.textSubColor }">任务标题</label>
        <input
          v-model="form.title"
          type="text"
          placeholder="输入任务标题"
          :style="inputStyle"
        />
      </div>

      <div class="form-row">
        <div class="form-group">
          <label :style="{ color: globalConfig.textSubColor }">预估耗时（小时）</label>
          <input
            v-model.number="form.estimated_time"
            type="number"
            min="0.5"
            max="24"
            step="0.5"
            :style="inputStyle"
          />
        </div>

        <div class="form-group">
          <label :style="{ color: globalConfig.textSubColor }">预估难度</label>
          <div class="difficulty-selector">
            <button
              v-for="n in 5"
              :key="n"
              :class="{ active: form.estimated_difficulty === n }"
              :style="difficultyStyle(n)"
              @click="form.estimated_difficulty = n"
            >
              {{ n }}
            </button>
          </div>
        </div>
      </div>

      <div class="form-group">
        <label :style="{ color: globalConfig.textSubColor }">任务描述（可选）</label>
        <textarea
          v-model="form.comment"
          rows="3"
          placeholder="添加任务描述或备注..."
          :style="inputStyle"
        />
      </div>

      <div class="form-group">
        <label class="checkbox-label">
          <input
            v-model="form.is_important"
            type="checkbox"
          />
          <span :style="{ color: globalConfig.textMainColor }">
            标记为 MIT（最重要任务）
          </span>
        </label>
      </div>

      <div class="form-actions">
        <button
          class="btn-cancel"
          :style="{ color: globalConfig.textSubColor }"
          @click="$emit('close')"
        >
          取消
        </button>
        <button
          class="btn-save"
          :style="{ backgroundColor: globalConfig.themePrimaryColor }"
          @click="handleSave"
        >
          保存
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, watch } from 'vue'
import type { Task, GlobalThemeConfig } from '../../types/rating'

const props = defineProps<{
  visible: boolean
  task?: Task | null
  globalConfig: GlobalThemeConfig
}>()

const emit = defineEmits<{
  close: []
  save: [taskData: Partial<Task>]
}>()

const form = reactive({
  title: '',
  estimated_time: 1,
  estimated_difficulty: 3,
  is_important: false,
  comment: ''
})

// 编辑模式时填充数据
watch(() => props.task, (newTask) => {
  if (newTask) {
    form.title = newTask.title
    form.estimated_time = newTask.estimated_time
    form.estimated_difficulty = newTask.estimated_difficulty
    form.is_important = newTask.is_important
    form.comment = newTask.comment || ''
  } else {
    form.title = ''
    form.estimated_time = 1
    form.estimated_difficulty = 3
    form.is_important = false
    form.comment = ''
  }
}, { immediate: true })

const inputStyle = {
  backgroundColor: 'rgba(255, 255, 255, 0.05)',
  border: '1px solid rgba(255, 255, 255, 0.1)',
  color: 'inherit',
  padding: '10px',
  borderRadius: '6px',
  width: '100%',
  fontSize: '14px'
}

const difficultyStyle = (n: number) => ({
  backgroundColor: form.estimated_difficulty === n
    ? props.globalConfig.themePrimaryColor
    : 'rgba(255, 255, 255, 0.05)',
  color: form.estimated_difficulty === n ? '#fff' : 'inherit',
  border: 'none',
  padding: '8px 14px',
  borderRadius: '6px',
  cursor: 'pointer',
  fontSize: '14px',
  fontWeight: 'bold'
})

const handleSave = () => {
  if (!form.title.trim()) {
    alert('请输入任务标题')
    return
  }

  emit('save', {
    title: form.title.trim(),
    estimated_time: form.estimated_time,
    estimated_difficulty: form.estimated_difficulty,
    is_important: form.is_important,
    comment: form.comment.trim() || undefined
  })
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal-content {
  width: 90%;
  max-width: 480px;
  padding: 24px;
  border-radius: 12px;
}

.modal-content h3 {
  font-size: 18px;
  font-weight: 600;
  margin-bottom: 20px;
}

.form-group {
  margin-bottom: 16px;
}

.form-group label {
  display: block;
  font-size: 14px;
  margin-bottom: 8px;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.difficulty-selector {
  display: flex;
  gap: 8px;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}

.checkbox-label input[type="checkbox"] {
  width: 18px;
  height: 18px;
  cursor: pointer;
}

.form-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  margin-top: 24px;
}

.btn-cancel,
.btn-save {
  padding: 10px 24px;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  cursor: pointer;
  transition: opacity 0.2s;
}

.btn-cancel {
  background: rgba(255, 255, 255, 0.1);
}

.btn-save {
  color: white;
}

.btn-cancel:hover,
.btn-save:hover {
  opacity: 0.9;
}
</style>