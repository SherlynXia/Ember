import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { 
  DailyRating, 
  Task, 
  DayDetail, 
  GlobalThemeConfig, 
  SaveRatingForm,
  AchievementTrendPoint
} from '../types/rating'

// 默认主题配置
const defaultThemeConfig: GlobalThemeConfig = {
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
}

/**
 * 评分数据 API 组合式函数
 * 以后端命令为基准：
 * - save_daily_rating: 保存每日评分
 * - get_daily_rating: 获取单日评分
 * - get_rating_list: 获取日期范围评分列表
 * - get_day_detail: 获取某天详情（评分+任务）
 * - delete_daily_rating: 删除评分
 * - create_task: 创建任务
 * - complete_task: 完成任务
 * - get_tasks_by_date: 获取某天任务列表
 */
export function useRatingApi() {
  // 状态
  const ratingList = ref<DailyRating[]>([])
  const currentRating = ref<DailyRating | null>(null)
  const currentTasks = ref<Task[]>([])
  const globalConfig = ref<GlobalThemeConfig>(defaultThemeConfig)
  const loading = ref(false)
  const error = ref<string | null>(null)
  const trendData = ref<AchievementTrendPoint[]>([])

  // ==================== 评分相关 API ====================

  /**
   * 保存每日评分
   * 对应后端命令: save_daily_rating
   */
  const saveDailyRating = async (form: SaveRatingForm): Promise<DailyRating> => {
    try {
      loading.value = true
      error.value = null
      
      const result = await invoke<DailyRating>('save_daily_rating', {
        dateStr: form.date,
        workType: form.work_type,
        energyScore: form.energy_score,
        valueScore: form.value_score,
        workHours: form.work_hours,
        moodEmoji: form.mood_emoji,
        comment: form.comment || null
      })
      
      // 更新本地列表
      const index = ratingList.value.findIndex(r => r.date === result.date)
      if (index >= 0) {
        ratingList.value[index] = result
      } else {
        ratingList.value.push(result)
      }
      
      currentRating.value = result
      return result
    } catch (err) {
      error.value = `保存评分失败: ${err}`
      throw err
    } finally {
      loading.value = false
    }
  }

  /**
   * 获取单日评分
   * 对应后端命令: get_daily_rating
   */
  const getDailyRating = async (date: string): Promise<DailyRating | null> => {
    try {
      error.value = null
      const result = await invoke<DailyRating>('get_daily_rating', {
        dateStr: date
      })
      currentRating.value = result
      return result
    } catch (err) {
      // 如果没有记录，返回 null 而不是报错
      if (String(err).includes('not found') || String(err).includes('未找到')) {
        currentRating.value = null
        return null
      }
      error.value = `获取评分失败: ${err}`
      throw err
    }
  }

  /**
   * 获取日期范围评分列表
   * 对应后端命令: get_rating_list
   */
  const getRatingList = async (startDate: string, endDate: string): Promise<DailyRating[]> => {
    try {
      loading.value = true
      error.value = null
      
      const result = await invoke<DailyRating[]>('get_rating_list', {
        startDate,
        endDate
      })
      
      ratingList.value = result
      return result
    } catch (err) {
      error.value = `获取评分列表失败: ${err}`
      throw err
    } finally {
      loading.value = false
    }
  }

  /**
   * 获取某天完整详情（评分 + 任务列表）
   * 对应后端命令: get_day_detail
   */
  const getDayDetail = async (date: string): Promise<DayDetail> => {
    try {
      loading.value = true
      error.value = null
      
      const result = await invoke<DayDetail>('get_day_detail', {
        dateStr: date
      })
      
      currentRating.value = result.rating
      currentTasks.value = result.tasks
      return result
    } catch (err) {
      error.value = `获取详情失败: ${err}`
      throw err
    } finally {
      loading.value = false
    }
  }

  /**
   * 删除评分
   * 对应后端命令: delete_daily_rating
   */
  const deleteDailyRating = async (date: string): Promise<void> => {
    try {
      loading.value = true
      error.value = null
      
      await invoke('delete_daily_rating', {
        dateStr: date
      })
      
      // 从本地列表移除
      ratingList.value = ratingList.value.filter(r => r.date !== date)
      
      if (currentRating.value?.date === date) {
        currentRating.value = null
      }
    } catch (err) {
      error.value = `删除评分失败: ${err}`
      throw err
    } finally {
      loading.value = false
    }
  }

  // ==================== 任务相关 API ====================

  /**
   * 创建任务
   * 对应后端命令: create_task
   */
  const createTask = async (params: {
    title: string
    estimatedTime: number
    estimatedDifficulty: number
    isImportant: boolean
  }): Promise<Task> => {
    try {
      error.value = null
      
      const result = await invoke<Task>('create_task', {
        title: params.title,
        estimatedTime: params.estimatedTime,
        estimatedDifficulty: params.estimatedDifficulty,
        isImportant: params.isImportant
      })
      
      return result
    } catch (err) {
      error.value = `创建任务失败: ${err}`
      throw err
    }
  }

  /**
   * 完成任务
   * 对应后端命令: complete_task
   */
  const completeTask = async (params: {
    taskId: number
    actualTime: number
    actualSatisfaction: number
    emojiMood: string
    energyScore: number
    valueScore: number
    comment?: string
  }): Promise<void> => {
    try {
      error.value = null
      
      await invoke('complete_task', {
        taskId: params.taskId,
        actualTime: params.actualTime,
        actualSatisfaction: params.actualSatisfaction,
        emojiMood: params.emojiMood,
        energyScore: params.energyScore,
        valueScore: params.valueScore,
        comment: params.comment || null
      })
    } catch (err) {
      error.value = `完成任务失败: ${err}`
      throw err
    }
  }

  /**
   * 获取某天任务列表
   * 对应后端命令: get_tasks_by_date
   */
  const getTasksByDate = async (date: string): Promise<Task[]> => {
    try {
      error.value = null
      
      const result = await invoke<Task[]>('get_tasks_by_date', {
        dateStr: date
      })
      
      currentTasks.value = result
      return result
    } catch (err) {
      error.value = `获取任务列表失败: ${err}`
      throw err
    }
  }

  // ==================== 统计相关 API ====================

  /**
   * 获取成就趋势数据（用于成长度趋势图）
   * 对应后端命令: get_achievement_trend
   */
  const getAchievementTrend = async (days: number = 30): Promise<AchievementTrendPoint[]> => {
    try {
      loading.value = true
      error.value = null
      
      const result = await invoke<AchievementTrendPoint[]>('get_achievement_trend', {
        days
      })
      
      trendData.value = result
      return result
    } catch (err) {
      error.value = `获取趋势数据失败: ${err}`
      throw err
    } finally {
      loading.value = false
    }
  }

  // ==================== 辅助方法 ====================

  /**
   * 加载初始数据
   */
  const loadInitialData = async () => {
    const today = new Date()
    const startDate = new Date(today.getFullYear(), 0, 1) // 今年1月1日
    const endDate = new Date(today.getFullYear(), 11, 31) // 今年12月31日
    
    await getRatingList(
      formatDate(startDate),
      formatDate(endDate)
    )
  }

  /**
   * 格式化日期为 YYYY-MM-DD
   */
  const formatDate = (date: Date): string => {
    const year = date.getFullYear()
    const month = String(date.getMonth() + 1).padStart(2, '0')
    const day = String(date.getDate()).padStart(2, '0')
    return `${year}-${month}-${day}`
  }

  /**
   * 获取今天的日期字符串
   */
  const getToday = (): string => {
    return formatDate(new Date())
  }

  return {
    // 状态
    ratingList,
    currentRating,
    currentTasks,
    globalConfig,
    loading,
    error,
    trendData,
    
    // 评分 API
    saveDailyRating,
    getDailyRating,
    getRatingList,
    getDayDetail,
    deleteDailyRating,
    
    // 任务 API
    createTask,
    completeTask,
    getTasksByDate,
    
    // 统计 API
    getAchievementTrend,
    
    // 辅助方法
    loadInitialData,
    formatDate,
    getToday
  }
}