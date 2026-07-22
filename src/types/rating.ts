export interface DailyRating {
  date: string
  status: number
  comment: string | null
  mood_emoji: string
  productivity_color: string
  work_hours: number
  achievement_score: number
  mit_completion_rate: number
  task_count_delta: number
  has_overdue: boolean
  has_milestone: boolean
}

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

export interface GridCell {
  cellIndex: number
  date: string | null
  record: DailyRating | null
}

export type RatingRes = { continueDays: number; list: DailyRating[] }