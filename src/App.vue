<template>
  <RatingRecord />
</template>

<script setup>
import { onMounted } from 'vue'
import RatingRecord from './views/RatingRecord.vue'
import { useUpdater } from './composables/useUpdater'

const { checkForUpdatesSilent } = useUpdater()

// 启动时自动检查更新
onMounted(() => {
  // 延迟3秒检查更新，避免影响启动速度
  setTimeout(() => {
    checkForUpdatesSilent()
  }, 3000)
})
</script>

<style>
* {
  margin: 0;
  padding: 0;
}
</style>