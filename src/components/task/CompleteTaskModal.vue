<template>
  <div class="modal-overlay" @click.self="$emit('close')">
    <div class="modal-content" :style="{ backgroundColor: globalConfig.cardBgColor }">
      <h3 :style="{ color: globalConfig.textMainColor }">完成任务</h3>

      <!-- 实际耗时 -->
      <div class="form-group">
        <label :style="{ color: globalConfig.textSubColor }">实际耗时（小时）</label>
        <input
          v-model.number="form.actual_time"
          type="number"
          min="0.5"
          max="24"
          step="0.5"
          :style="inputStyle"
        />
      </div>

      <!-- 满意度 -->
      <div class="form-group">
        <label :style="{ color: globalConfig.textSubColor }">满意度</label>
        <div class="satisfaction-selector">
          <button
            v-for="n in 5"
            :key="n"
            :class="{ active: form.actual_satisfaction === n }"
            :style="satisfactionStyle(n)"
            @click="form.actual_satisfaction = n"
          >
            {{ n }}
          </button>
        </div>
      </div>

      <!-- 心情 -->
      <div class="form-group">
        <label :style="{ color: globalConfig.textSubColor }">心情</label>
        <div class="mood-selector">
          <button
            v-for="mood in moodOptions"
            :key="mood.emoji"
            :class="{ active: form.emoji_mood === mood.emoji }"
            :style="moodStyle(mood.emoji)"
            @click="form.emoji_mood = mood.emoji"
          >
            {{ mood.emoji }}
          </button>
        </div>
      </div>

      <!-- 精力评分 -->
      <div class="form-group">
        <label :style="{ color: globalConfig.textSubColor }">精力消耗 (1-5)</label>
        <div class="score-selector">
          <button
            v-for="n in 5"
            :key="n"
            :class="{ active: form.energy_score === n }"
            :style="scoreStyle(n, 'energy')"
            @click="form.energy_score = n"
          >
            {{ n }}
          </button>
        </div>
      </div>

      <!-- 价值评分 -->
      <div class="form-group">
        <label :style="{ color: globalConfig.textSubColor }">价值产出 (1-5)</label>
        <div class="score-selector">
          <button
            v-for="n in 5"
            :key="n"
            :class="{ active: form.value_score === n }"
            :style="scoreStyle(n, 'value')"
            @click="form.value_score = n"
          >
            {{ n }}
          </button>
        </div>
      </div>

      <!-- 评语 -->
      <div class="form-group">
        <label :style="{ color: globalConfig.textSubColor }">评语（可选）</label>
        <textarea
          v-model="form.comment"
          rows="3"
          placeholder="记录一下完成情况..."
          :style="inputStyle"
        />
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
          @click="handleSubmit"
        >
          完成
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive } from 'vue'
import type { GlobalThemeConfig } from '../../types/rating'

const props = defineProps<{
  visible: boolean
  taskId: number
  globalConfig: GlobalThemeConfig
}>()

const emit = defineEmits<{
  close: []
  complete: [data: {
    taskId: number
    actualTime: number
    actualSatisfaction: number
    emojiMood: string
    energyScore: number
    valueScore: number
    comment?: string
  }]
}>()

const form = reactive({
  actual_time: 1,
  actual_satisfaction: 3,
  emoji_mood: '😊',
  energy_score: 3,
  value_score: 3,
  comment: ''
})

const moodOptions = [
  { emoji: '😫', label: '疲惫' },
  { emoji: '😔', label: '低落' },
  { emoji: '😐', label: '一般' },
  { emoji: '😊', label: '不错' },
  { emoji: '🤩', label: '超棒' }
]

const inputStyle = {
  backgroundColor: 'rgba(255, 255, 255, 0.05)',
  border: '1px solid rgba(255, 255, 255, 0.1)',
  color: 'inherit',
  padding: '10px',
  borderRadius: '6px',
  width: '100%',
  fontSize: '14px',
  resize: 'vertical' as const
}

const satisfactionStyle = (n: number) => ({
  backgroundColor: form.actual_satisfaction === n
    ? props.globalConfig.themePrimaryColor
    : 'rgba(255, 255, 255, 0.05)',
  color: form.actual_satisfaction === n ? '#fff' : 'inherit',
  border: 'none',
  padding: '8px 14px',
  borderRadius: '6px',
  cursor: 'pointer',
  fontSize: '14px',
  fontWeight: 'bold'
})

const moodStyle = (emoji: string) => ({
  backgroundColor: form.emoji_mood === emoji
    ? props.globalConfig.themePrimaryColor
    : 'rgba(255, 255, 255, 0.05)',
  border: 'none',
  padding: '8px 12px',
  borderRadius: '6px',
  cursor: 'pointer',
  fontSize: '20px'
})

const scoreStyle = (n: number, type: 'energy' | 'value') => {
  const isActive = type === 'energy' ? form.energy_score === n : form.value_score === n
  return {
    backgroundColor: isActive
      ? type === 'energy' ? '#ef4444' : '#22c55e'
      : 'rgba(255, 255, 255, 0.05)',
    color: isActive ? '#fff' : 'inherit',
    border: 'none',
    padding: '8px 14px',
    borderRadius: '6px',
    cursor: 'pointer',
    fontSize: '14px',
    fontWeight: 'bold'
  }
}

const handleSubmit = () => {
  emit('complete', {
    taskId: props.taskId,
    actualTime: form.actual_time,
    actualSatisfaction: form.actual_satisfaction,
    emojiMood: form.emoji_mood,
    energyScore: form.energy_score,
    valueScore: form.value_score,
    comment: form.comment || undefined
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
  max-height: 90vh;
  overflow-y: auto;
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

.satisfaction-selector,
.mood-selector,
.score-selector {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
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