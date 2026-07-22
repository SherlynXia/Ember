import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { DailyRating, GlobalThemeConfig, RatingRes } from '../types/rating'

export function useRatingApi() {
  const ratingList = ref<DailyRating[]>([])
  const globalConfig = ref<GlobalThemeConfig>({
    themePrimaryColor: '#3b82f6',
    emptyCellColor: '#222226',
    tabActiveBg: 'rgba(59, 130, 246, 0.2)',
    textMainColor: '#f5f5f5',
    textSubColor: '#9ca3af',
    cardBgColor: '#1e1e22',
    pageBgColor: '#121214',
    themeBorderColor: '#333338',
    tooltipBgColor: '#0a0a0c',
    tooltipTextColor: '#f5f5f5'
  })
  const continueDays = ref(0)
  const loadError = ref(false)
  const loading = ref(false)

  let fetchLock = false

  const fetchGlobalTheme = async () => {
    const data = await invoke<GlobalThemeConfig>("cmd_get_theme_config")
    console.log('✅ 主题配色加载成功：', data)
    globalConfig.value = data
  }

  const fetchDailyRating = async () => {
    const res = await invoke<RatingRes>("cmd_get_rating_list")
    console.log('✅ 评分数据加载成功：', res)
    ratingList.value = res.list
    continueDays.value = res.continueDays
  }

  const fetchAllData = async () => {
    if (fetchLock || loading.value) return
    fetchLock = true
    loading.value = true
    loadError.value = false
    try {
      await Promise.all([
        fetchGlobalTheme().catch(e => console.error('配色配置加载失败', e)),
        fetchDailyRating().catch(e => console.error('评分记录加载失败', e))
      ])
    } catch (err) {
      loadError.value = true
      console.error('❌ Tauri数据调用失败：', err)
    } finally {
      loading.value = false
      fetchLock = false
    }
  }

  return {
    ratingList,
    globalConfig,
    continueDays,
    loading,
    loadError,
    fetchAllData
  }
}