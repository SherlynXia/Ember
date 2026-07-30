<template>
  <Teleport to="body">
    <Transition name="modal">
      <div v-if="visible" class="modal-overlay" @click.self="handleClose">
        <div class="modal-container" :style="{ backgroundColor: globalConfig.cardBgColor }">
          <!-- 头部 -->
          <div class="modal-header">
            <h3 :style="{ color: globalConfig.textMainColor }">
              {{ isEdit ? '编辑评分' : '记录今天' }} - {{ formatDisplayDate(date) }}
            </h3>
            <button class="close-btn" @click="handleClose" :style="{ color: globalConfig.textSubColor }">
              ×
            </button>
          </div>

          <div class="modal-body">
            <!-- 1. Emoji 心情选择 -->
            <div class="form-section">
              <label :style="{ color: globalConfig.textMainColor }">
                今日心情 <span class="required">*</span>
              </label>
              <div class="emoji-selector">
                <button
                  v-for="mood in moodOptions"
                  :key="mood.emoji"
                  type="button"
                  class="emoji-btn"
                  :class="{ active: form.mood_emoji === mood.emoji }"
                  :style="{ 
                    backgroundColor: form.mood_emoji === mood.emoji ? globalConfig.themePrimaryColor : 'transparent',
                    borderColor: globalConfig.themeBorderColor 
                  }"
                  @click="form.mood_emoji = mood.emoji"
                >
                  <span class="emoji">{{ mood.emoji }}</span>
                  <span class="emoji-label" :style="{ color: globalConfig.textSubColor }">
                    {{ mood.label }}
                  </span>
                </button>
              </div>
            </div>

            <!-- 2. 工作状态选择（色相） -->
            <div class="form-section">
              <label :style="{ color: globalConfig.textMainColor }">
                工作状态 <span class="required">*</span>
              </label>
              <div class="work-type-selector">
                <button
                  v-for="type in workTypeOptions"
                  :key="type.value"
                  type="button"
                  class="work-type-btn"
                  :class="{ active: form.work_type === type.value }"
                  :style="{ 
                    backgroundColor: form.work_type === type.value ? type.color + '40' : 'transparent',
                    borderColor: type.color,
                    color: globalConfig.textMainColor
                  }"
                  @click="form.work_type = type.value"
                >
                  <span class="color-dot" :style="{ backgroundColor: type.color }"></span>
                  <span class="type-label">{{ type.label }}</span>
                  <span class="type-desc" :style="{ color: globalConfig.textSubColor }">
                    {{ type.desc }}
                  </span>
                </button>
              </div>
            </div>

            <!-- 3. 二维评分图表（能量-价值） -->
            <div class="form-section">
              <label :style="{ color: globalConfig.textMainColor }">
                能量-价值评分 <span class="required">*</span>
              </label>
              <div class="chart-container" :style="{ backgroundColor: globalConfig.pageBgColor }">
                <!-- Y轴标签 -->
                <div class="y-axis-label" :style="{ color: globalConfig.textSubColor }">
                  精力消耗 ↑
                </div>
                
                <!-- 图表主体 -->
                <div class="chart-grid">
                  <div
                    v-for="y in 5"
                    :key="`y-${y}`"
                    class="chart-row"
                  >
                    <button
                      v-for="x in 5"
                      :key="`${x}-${6-y}`"
                      type="button"
                      class="chart-cell"
                      :class="{ 
                        active: form.energy_score === (6-y) && form.value_score === x,
                        'golden-zone': x >= 4 && (6-y) <= 2
                      }"
                      :style="{
                        backgroundColor: getCellColor(x, 6-y),
                        borderColor: globalConfig.themeBorderColor
                      }"
                      @click="selectEnergyValue(x, 6-y)"
                    >
                      <span v-if="form.energy_score === (6-y) && form.value_score === x" class="check-mark">
                        ✓
                      </span>
                    </button>
                  </div>
                </div>

                <!-- X轴标签 -->
                <div class="x-axis-label" :style="{ color: globalConfig.textSubColor }">
                  → 贡献价值
                </div>

                <!-- 选中值显示 -->
                <div class="selected-value" :style="{ color: globalConfig.textMainColor }">
                  精力消耗: <strong>{{ form.energy_score }}</strong> | 
                  贡献价值: <strong>{{ form.value_score }}</strong>
                  <span v-if="isGoldenZone" class="golden-badge">🌟 黄金区</span>
                </div>
              </div>
              <div class="chart-legend" :style="{ color: globalConfig.textSubColor }">
                <span>左下角：低消耗 + 高价值 = 黄金区</span>
                <span>右上角：高消耗 + 低价值 = 垃圾区</span>
              </div>
            </div>

            <!-- 4. 工作时长 -->
            <div class="form-section">
              <label :style="{ color: globalConfig.textMainColor }">
                工作时长（小时） <span class="required">*</span>
              </label>
              <input
                v-model.number="form.work_hours"
                type="number"
                min="0"
                max="24"
                step="0.5"
                class="form-input"
                :style="{ 
                  backgroundColor: globalConfig.pageBgColor,
                  color: globalConfig.textMainColor,
                  borderColor: globalConfig.themeBorderColor
                }"
                placeholder="请输入今天的工作时长"
              />
            </div>

            <!-- 5. 评语 -->
            <div class="form-section">
              <label :style="{ color: globalConfig.textMainColor }">
                今日评语 <span class="optional">（可选，最多50字）</span>
              </label>
              <textarea
                v-model="form.comment"
                rows="3"
                maxlength="50"
                class="form-textarea"
                :style="{ 
                  backgroundColor: globalConfig.pageBgColor,
                  color: globalConfig.textMainColor,
                  borderColor: globalConfig.themeBorderColor
                }"
                placeholder="记录今天的感想、收获或反思..."
              ></textarea>
              <div class="char-count" :style="{ color: globalConfig.textSubColor }">
                {{ form.comment.length }}/50
              </div>
            </div>
          </div>

          <!-- 底部按钮 -->
          <div class="modal-footer">
            <button
              type="button"
              class="btn btn-secondary"
              :style="{ 
                backgroundColor: 'transparent',
                color: globalConfig.textSubColor,
                borderColor: globalConfig.themeBorderColor
              }"
              @click="handleClose"
            >
              取消
            </button>
            <button
              type="button"
              class="btn btn-primary"
              :style="{ 
                backgroundColor: globalConfig.themePrimaryColor,
                color: '#fff'
              }"
              :disabled="!isFormValid || saving"
              @click="handleSave"
            >
              {{ saving ? '保存中...' : (isEdit ? '更新' : '保存') }}
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import type { GlobalThemeConfig, SaveRatingForm, WorkTypeOption, MoodOption } from '../../types/rating'

const props = defineProps<{
  visible: boolean
  date: string
  globalConfig: GlobalThemeConfig
  initialData?: Partial<SaveRatingForm> | null
  readonly?: boolean  // 是否只读（往期）
}>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'save', form: SaveRatingForm): void
}>()

// 工作状态选项
const workTypeOptions: WorkTypeOption[] = [
  { value: 'green', label: '高产日', desc: '效率满满，成果丰硕', color: '#22c55e' },
  { value: 'blue', label: '学习日', desc: '沉浸学习，积累成长', color: '#3b82f6' },
  { value: 'orange', label: '低效日', desc: '忙碌但成果有限', color: '#fb923c' },
  { value: 'gray', label: '休息日', desc: '放松身心，调整状态', color: '#888888' }
]

// 心情选项
const moodOptions: MoodOption[] = [
  { emoji: '😊', label: '爽', desc: '有成就感' },
  { emoji: '😐', label: '一般', desc: '平淡' },
  { emoji: '😫', label: '累', desc: '没进展' },
  { emoji: '🥳', label: '超常', desc: '超常发挥' }
]

// 表单数据
const form = ref<SaveRatingForm>({
  date: props.date,
  work_type: 'green',
  energy_score: 3,
  value_score: 3,
  work_hours: 8,
  mood_emoji: '😊',
  comment: ''
})

const saving = ref(false)

// 是否编辑模式
const isEdit = computed(() => !!props.initialData)

// 是否在黄金区（高价值 + 低消耗）
const isGoldenZone = computed(() => {
  return form.value.value_score >= 4 && form.value.energy_score <= 2
})

// 表单验证
const isFormValid = computed(() => {
  return form.value.mood_emoji &&
         form.value.work_type &&
         form.value.energy_score >= 1 && form.value.energy_score <= 5 &&
         form.value.value_score >= 1 && form.value.value_score <= 5 &&
         form.value.work_hours > 0 && form.value.work_hours <= 24
})

// 监听日期和初始数据变化
watch(() => props.date, (newDate) => {
  form.value.date = newDate
})

watch(() => props.initialData, (data) => {
  if (data) {
    form.value = {
      date: data.date || props.date,
      work_type: data.work_type || 'green',
      energy_score: data.energy_score || 3,
      value_score: data.value_score || 3,
      work_hours: data.work_hours || 8,
      mood_emoji: data.mood_emoji || '😊',
      comment: data.comment || ''
    }
  } else {
    // 重置为默认值
    form.value = {
      date: props.date,
      work_type: 'green',
      energy_score: 3,
      value_score: 3,
      work_hours: 8,
      mood_emoji: '😊',
      comment: ''
    }
  }
}, { immediate: true })

// 选择能量-价值
const selectEnergyValue = (value: number, energy: number) => {
  if (props.readonly) return
  form.value.value_score = value
  form.value.energy_score = energy
}

// 获取图表单元格颜色
const getCellColor = (value: number, energy: number) => {
  const efficiency = value / energy
  // 根据效率计算饱和度，效率越高颜色越深
  const saturation = 0.3 + (Math.min(efficiency, 2) / 2) * 0.7
  const alpha = saturation.toFixed(2)
  return `rgba(59, 130, 246, ${alpha})`
}

// 格式化显示日期
const formatDisplayDate = (dateStr: string): string => {
  const date = new Date(dateStr)
  const month = date.getMonth() + 1
  const day = date.getDate()
  const weekDays = ['日', '一', '二', '三', '四', '五', '六']
  const weekDay = weekDays[date.getDay()]
  return `${month}月${day}日 周${weekDay}`
}

// 关闭弹窗
const handleClose = () => {
  emit('close')
}

// 保存
const handleSave = async () => {
  if (!isFormValid.value || props.readonly) return
  
  saving.value = true
  try {
    emit('save', { ...form.value })
  } finally {
    saving.value = false
  }
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-color: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: 20px;
}

.modal-container {
  width: 100%;
  max-width: 520px;
  max-height: 90vh;
  border-radius: 12px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.modal-header h3 {
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

.modal-body {
  padding: 24px;
  overflow-y: auto;
  flex: 1;
}

.form-section {
  margin-bottom: 24px;
}

.form-section label {
  display: block;
  font-size: 14px;
  font-weight: 500;
  margin-bottom: 12px;
}

.required {
  color: #ef4444;
}

.optional {
  font-size: 12px;
  font-weight: normal;
  opacity: 0.7;
}

/* Emoji 选择器 */
.emoji-selector {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 12px;
}

.emoji-btn {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 16px 8px;
  border: 2px solid;
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.2s;
  background: none;
}

.emoji-btn:hover {
  transform: translateY(-2px);
}

.emoji-btn.active {
  border-width: 2px;
}

.emoji {
  font-size: 32px;
  margin-bottom: 4px;
}

.emoji-label {
  font-size: 12px;
}

/* 工作状态选择器 */
.work-type-selector {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.work-type-btn {
  display: flex;
  align-items: center;
  padding: 12px 16px;
  border: 2px solid;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
  background: none;
  text-align: left;
}

.work-type-btn:hover {
  transform: translateX(4px);
}

.color-dot {
  width: 16px;
  height: 16px;
  border-radius: 50%;
  margin-right: 12px;
  flex-shrink: 0;
}

.type-label {
  font-weight: 500;
  margin-right: 8px;
}

.type-desc {
  font-size: 12px;
}

/* 二维评分图表 */
.chart-container {
  padding: 20px;
  border-radius: 10px;
  position: relative;
}

.y-axis-label {
  position: absolute;
  left: 4px;
  top: 50%;
  transform: rotate(-90deg) translateX(50%);
  transform-origin: left center;
  font-size: 12px;
  white-space: nowrap;
}

.x-axis-label {
  text-align: center;
  margin-top: 8px;
  font-size: 12px;
}

.chart-grid {
  display: flex;
  flex-direction: column;
  gap: 4px;
  margin: 0 20px;
}

.chart-row {
  display: flex;
  gap: 4px;
  justify-content: center;
}

.chart-cell {
  width: 48px;
  height: 48px;
  border: 2px solid;
  border-radius: 6px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s;
  background: none;
}

.chart-cell:hover {
  transform: scale(1.1);
}

.chart-cell.active {
  border-color: #3b82f6;
  border-width: 3px;
}

.chart-cell.golden-zone {
  box-shadow: inset 0 0 0 2px rgba(251, 191, 36, 0.5);
}

.check-mark {
  font-size: 20px;
  color: #ffffff;
  font-weight: bold;
  text-shadow: 0 1px 3px rgba(0, 0, 0, 0.5);
  filter: drop-shadow(0 1px 2px rgba(0, 0, 0, 0.3));
}

.selected-value {
  text-align: center;
  margin-top: 16px;
  font-size: 14px;
}

.selected-value strong {
  color: #3b82f6;
  font-size: 16px;
}

.golden-badge {
  margin-left: 12px;
  background: linear-gradient(135deg, #fbbf24, #f59e0b);
  color: #000;
  padding: 2px 8px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 600;
}

.chart-legend {
  display: flex;
  justify-content: space-between;
  margin-top: 8px;
  font-size: 11px;
}

/* 表单输入 */
.form-input,
.form-textarea {
  width: 100%;
  padding: 12px 16px;
  border: 1px solid;
  border-radius: 8px;
  font-size: 14px;
  outline: none;
  transition: border-color 0.2s;
  box-sizing: border-box;
}

.form-input:focus,
.form-textarea:focus {
  border-color: #3b82f6;
}

.form-textarea {
  resize: vertical;
  min-height: 80px;
}

.char-count {
  text-align: right;
  font-size: 12px;
  margin-top: 4px;
}

/* 底部按钮 */
.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  padding: 16px 24px;
  border-top: 1px solid rgba(255, 255, 255, 0.1);
}

.btn {
  padding: 10px 24px;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  border: 1px solid;
}

.btn:hover:not(:disabled) {
  opacity: 0.9;
  transform: translateY(-1px);
}

.btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* 动画 */
.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.3s ease;
}

.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-active .modal-container,
.modal-leave-active .modal-container {
  transition: transform 0.3s ease;
}

.modal-enter-from .modal-container,
.modal-leave-to .modal-container {
  transform: scale(0.95);
}

@media screen and (max-width: 600px) {
  .modal-overlay {
    padding: 0;
    align-items: flex-end;
  }
  
  .modal-container {
    max-height: 85vh;
    border-radius: 16px 16px 0 0;
  }
  
  .emoji-selector {
    grid-template-columns: repeat(2, 1fr);
  }
  
  .chart-cell {
    width: 40px;
    height: 40px;
  }
}
</style>