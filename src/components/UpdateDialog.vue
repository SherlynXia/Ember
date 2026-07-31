<template>
  <div v-if="visible" class="dialog-overlay" @click="handleOverlayClick">
    <div class="dialog-content" @click.stop>
      <div class="dialog-header">
        <h3>{{ title }}</h3>
      </div>
      <div class="dialog-body">
        <p>{{ message }}</p>
      </div>
      <div class="dialog-footer">
        <button v-if="showCancel" class="btn-cancel" @click="handleCancel">取消</button>
        <button class="btn-confirm" @click="handleConfirm">确定</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// defineProps 和 defineEmits 在 <script setup> 中自动可用，无需导入

const props = defineProps<{
  visible: boolean
  title: string
  message: string
  showCancel?: boolean
}>()

const emit = defineEmits<{
  confirm: []
  cancel: []
}>()

const handleConfirm = () => {
  emit('confirm')
}

const handleCancel = () => {
  emit('cancel')
}

const handleOverlayClick = () => {
  if (props.showCancel) {
    emit('cancel')
  } else {
    emit('confirm')
  }
}
</script>

<style scoped>
.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 10000;
}

.dialog-content {
  background: #1e293b;
  border-radius: 12px;
  padding: 24px;
  min-width: 300px;
  max-width: 400px;
  box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.3);
}

.dialog-header h3 {
  margin: 0 0 16px 0;
  color: #e5e7eb;
  font-size: 18px;
}

.dialog-body p {
  margin: 0 0 24px 0;
  color: #9ca3af;
  line-height: 1.5;
}

.dialog-footer {
  display: flex;
  gap: 12px;
  justify-content: flex-end;
}

.btn-cancel, .btn-confirm {
  padding: 8px 16px;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 14px;
  transition: all 0.2s;
}

.btn-cancel {
  background: rgba(255, 255, 255, 0.1);
  color: #e5e7eb;
}

.btn-cancel:hover {
  background: rgba(255, 255, 255, 0.2);
}

.btn-confirm {
  background: #3b82f6;
  color: white;
}

.btn-confirm:hover {
  background: #2563eb;
}
</style>