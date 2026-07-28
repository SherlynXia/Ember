// ==================== 数据模型类型 ====================

/** 每日评分数据 */
export interface DailyRating {
  date: string
  // === 用户填写的核心字段 ===
  work_type: 'green' | 'blue' | 'orange' | 'gray'  // 工作状态类型
  energy_score: number           // 精力消耗分 1-5（纵轴）
  value_score: number            // 贡献价值分 1-5（横轴）
  work_hours: number             // 有效专注时长（小时）
  mood_emoji: 'ðŸ˜Š' | 'ðŸ˜�' | 'ðŸ˜«' | 'ðŸ¥³'  // 心情表情
  comment: string | null         // 每日评语

  // === 后端计算生成的字段 ===
  productivity_color: string     // 最终RGBA颜色字符串
  achievement_score: number      // 综合成就分 0-100

  // === 扩展字段（自动统计）===
  status: number
  mit_completion_rate: number
  task_count_delta: number
  has_overdue: boolean
  has_milestone: boolean
}

/** 任务数据 */
export interface Task {
  id: number
  title: string
  estimated_time: number
  estimated_difficulty: number
  actual_time: number | null
  actual_satisfaction: number | null
  is_completed: boolean
  completion_date: string | null
  is_important: boolean
  emoji_mood: string | null
  energy_score: number | null
  value_score: number | null
  comment: string | null
  created_at: string
}

/** 某天完整详情 */
export interface DayDetail {
  rating: DailyRating | null
  tasks: Task[]
}

// ==================== UI 组件类型 ====================

/** 热力图单元格 */
export interface GridCell {
  cellIndex: number
  date: string | null
  record: DailyRating | null
}

/** 全局主题配置 */
export interface GlobalThemeConfig {
  themePrimaryColor: string
  emptyCellColor: string
  tabActiveBg: string
  textMainColor: string
  textSubColor: string
  cardBgColor: string
  pageBgColor: string
  themeBorderColor: string
  tooltipBgColor: string
  tooltipTextColor: string
}

// ==================== 表单类型 ====================

/** 保存评分表单数据 */
export interface SaveRatingForm {
  date: string
  work_type: 'green' | 'blue' | 'orange' | 'gray'
  energy_score: number
  value_score: number
  work_hours: number
  mood_emoji: 'ðŸ˜Š' | 'ðŸ˜�' | 'ðŸ˜«' | 'ðŸ¥³'
  comment: string
}

/** 工作状态选项 */
export interface WorkTypeOption {
  value: 'green' | 'blue' | 'orange' | 'gray'
  label: string
  desc: string
  color: string
}

/** 心情选项 */
export interface MoodOption {
  emoji: 'ðŸ˜Š' | 'ðŸ˜�' | 'ðŸ˜«' | 'ðŸ¥³'
  label: string
  desc: string
}