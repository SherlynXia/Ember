<template>
  <div class="pet-container">
    <div
      class="pet-sprite"
      @mousedown="startDrag"
    >
      <template v-if="currentFrame">
        <img
          :src="currentFrame"
          :alt="state"
          class="pet-frame"
          draggable="false"
        />
      </template>
      <template v-else>
        <div class="pet-placeholder">
          {{ state === 'stand' ? '🐱' : '🐾' }}
        </div>
      </template>
    </div>
  </div>
</template>

<script setup lang="ts">
import { usePet } from '../../composables/usePet'

const { state, currentFrame, startDrag } = usePet()
</script>

<style scoped>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

.pet-container {
  width: 250px;
  height: 250px;
  background: transparent !important;
  overflow: visible;
  cursor: grab;
}

.pet-sprite {
  width: 250px;
  height: 250px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.pet-frame {
  width: 100%;
  height: 100%;
  object-fit: contain;
  pointer-events: none;
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
    transform: translateY(-10px);
  }
}
</style>