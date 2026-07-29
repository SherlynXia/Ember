import { computed, Ref } from 'vue'
import type { DailyRating, GridCell, MonthLabel } from '../types/rating'

export const DAYS_PER_WEEK = 7 // 每列7天（周一到周日）
export const TOTAL_WEEKS = 53 // 显示53周（约一年）
export const TOTAL_CELL = DAYS_PER_WEEK * TOTAL_WEEKS

/**
 * 获取日期是星期几（0=周日, 1=周一, ... 6=周六）
 * 转换为周一为起始：周一=0, 周二=1, ... 周日=6
 */
function getDayOfWeek(date: Date): number {
  const day = date.getDay()
  return day === 0 ? 6 : day - 1
}

/**
 * 格式化日期为 YYYY-MM-DD
 */
function formatDate(date: Date): string {
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  return `${year}-${month}-${day}`
}

/**
 * 获取指定年份的7月1日
 */
function getJulyFirst(year: number): Date {
  return new Date(year, 6, 1) // 月份从0开始，6=7月
}

/**
 * 获取指定年份的次年6月30日
 */
function getNextJune30th(year: number): Date {
  return new Date(year + 1, 5, 30) // 5=6月
}

/**
 * 获取今天的日期字符串
 */
function getToday(): string {
  return formatDate(new Date())
}

export function useRatingHeatmap(ratingList: Ref<DailyRating[]>) {
  // 当前年份（基于7月1日-次年6月30日）
  const currentYear = computed(() => {
    const now = new Date()
    const currentYearJuly = getJulyFirst(now.getFullYear())
    return now >= currentYearJuly ? now.getFullYear() : now.getFullYear() - 1
  })

  // 学年起始日期（7月1日）
  const yearStartDate = computed(() => getJulyFirst(currentYear.value))

  // 学年结束日期（次年6月30日）
  const yearEndDate = computed(() => getNextJune30th(currentYear.value))

  // 7月1日是星期几
  const startDayOfWeek = computed(() => getDayOfWeek(yearStartDate.value))

  // 今天
  const today = computed(() => getToday())

  // 生成完整的日期网格
  const gridCellList = computed<GridCell[]>(() => {
    const cells: GridCell[] = []

    // 从7月1日开始，往前推 startDayOfWeek 天到周一
    const firstCellDate = new Date(yearStartDate.value)
    firstCellDate.setDate(firstCellDate.getDate() - startDayOfWeek.value)

    // 生成 53周 × 7天 = 371 个格子
    for (let week = 0; week < TOTAL_WEEKS; week++) {
      for (let day = 0; day < DAYS_PER_WEEK; day++) {
        const cellDate = new Date(firstCellDate)
        cellDate.setDate(firstCellDate.getDate() + week * 7 + day)

        const dateStr = formatDate(cellDate)
        const record = ratingList.value.find(r => r.date === dateStr)

        // 判断是否在学年内（7月1日 - 次年6月30日）
        const isInYearRange = cellDate >= yearStartDate.value && cellDate <= yearEndDate.value

        // 判断是否是今天或之前的日期
        const isPastOrToday = dateStr <= today.value

        cells.push({
          cellIndex: week * DAYS_PER_WEEK + day,
          date: isInYearRange ? dateStr : null,
          record: record || null,
          weekIndex: week,
          dayIndex: day,
          isInYearRange,
          isPastOrToday
        })
      }
    }

    return cells
  })

  // 计算月份标签位置
  const monthLabels = computed<MonthLabel[]>(() => {
    const labels: MonthLabel[] = []
    const startDate = new Date(yearStartDate.value)
    startDate.setDate(startDate.getDate() - startDayOfWeek.value)

    // 遍历每个月
    for (let monthOffset = 0; monthOffset <= 12; monthOffset++) {
      const monthDate = new Date(yearStartDate.value)
      monthDate.setMonth(monthDate.getMonth() + monthOffset)

      // 找到该月1日在网格中的位置
      const firstDayOfMonth = new Date(monthDate.getFullYear(), monthDate.getMonth(), 1)

      // 如果超出范围则跳过
      if (firstDayOfMonth > yearEndDate.value) continue

      // 计算该月1日距离起始日期的天数
      const daysDiff = Math.floor((firstDayOfMonth.getTime() - startDate.getTime()) / (1000 * 60 * 60 * 24))
      const weekIndex = Math.floor(daysDiff / 7)

      // 只添加在有效范围内的标签
      if (weekIndex >= 0 && weekIndex < TOTAL_WEEKS) {
        labels.push({
          month: monthDate.getMonth() + 1,
          weekIndex
        })
      }
    }

    return labels
  })

  // 统计信息 - 只统计到今天为止的记录
  const totalRecordDays = computed(() => {
    const yearStart = formatDate(yearStartDate.value)
    return new Set(
      ratingList.value
        .filter(r => r.date >= yearStart && r.date <= today.value)
        .map(i => i.date)
    ).size
  })

  const totalRecordCount = computed(() => {
    const yearStart = formatDate(yearStartDate.value)
    return ratingList.value.filter(r => r.date >= yearStart && r.date <= today.value).length
  })

  // 底部点阵图（保留原有功能）
  const DOT_COL = 12
  const dotCount = DOT_COL * 10
  const dotTotalList = computed(() => Array.from({ length: dotCount }, (_, i) => i))
  const dotGridStyle = computed(() => ({
    gridTemplateColumns: `repeat(${DOT_COL}, 1fr)`
  }))

  return {
    currentYear,
    yearStartDate,
    yearEndDate,
    today,
    totalRecordDays,
    totalRecordCount,
    gridCellList,
    monthLabels,
    dotTotalList,
    dotGridStyle
  }
}