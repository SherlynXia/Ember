<template>
  <div
    class="pet-container"
    :style="containerStyle"
    @mousedown="startDrag"
  >
    <div class="pet-sprite" :class="[`state-${state}`]">
      <img
        v-if="currentFrame && !usePlaceholder"
        :src="currentFrame"
        :alt="state"
        draggable="false"
      />
      <div v-else class="pet-placeholder">
        {{ placeholderEmoji }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { usePet } from '../../composables/usePet'

const { state, position, direction, currentFrame, usePlaceholder, startDrag } = usePet()

// 容器样式
const containerStyle = computed(() => ({
  left: `${position.x}px`,
  top: `${position.y}px`,
  transform: `scaleX(${direction === 'left' ? -1 : 1})`,
}))

// 根据状态显示不同表情
const placeholderEmoji = computed(() => {
  switch (state.value) {
    case 'walk-left':
    case 'walk-right':
      return '🐾'
    case 'stand':
    default:
      return '🐱'
  }
})
</script>

<style scoped>
.pet-container {
  position: fixed;
  z-index: 9999;
  cursor: grab;
  user-select: none;
  pointer-events: auto;
  transition: none;
}

.pet-container:active {
  cursor: grabbing;
}

.pet-sprite {
  width: 250px;
  height: 250px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.pet-sprite img {
  width: 100%;
  height: 100%;
  object-fit: contain;
  image-rendering: pixelated;
}

.pet-placeholder {
  font-size: 150px;
  line-height: 1;
  animation: bounce 2s infinite;
}

@keyframes bounce {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-5px);
  }
}

.state-stand .pet-placeholder {
  animation: breathe 3s infinite;
}

@keyframes breathe {
  0%, 100% {
    transform: scale(1);
  }
  50% {
    transform: scale(1.05);
  }
}
</style>