import { computed, Ref } from 'vue'
import type { DailyRating, GridCell } from '../types/rating'

export const HEAT_ROW = 6
export const HEAT_COL = 60
export const TOTAL_CELL = HEAT_ROW * HEAT_COL
export const DOT_COL = 12
export const dotCount = DOT_COL * 10

export function useRatingHeatmap(ratingList: Ref<DailyRating[]>) {
  const totalRecordDays = computed(() => new Set(ratingList.value.map(i => i.date)).size)
  const totalRecordCount = computed(() => ratingList.value.length)
  const hasOverflowRecord = computed(() => ratingList.value.length > TOTAL_CELL)

  const fullDisplayMonthList = computed(() => {
    if (!ratingList.value.length) return []
    const dates = ratingList.value.map(item => new Date(item.date))
    const minDate = new Date(Math.min(...dates.map(d => d.getTime())))
    const maxDate = new Date(Math.max(...dates.map(d => d.getTime())))
    const months: number[] = []
    let current = new Date(minDate)
    while (current <= maxDate) {
      months.push(current.getMonth() + 1)
      current.setMonth(current.getMonth() + 1)
    }
    return months
  })

  const displayMonthList = computed(() => {
    const existMonth = new Set<number>()
    ratingList.value.forEach(item => {
      const m = Number(item.date.split('-')[1])
      existMonth.add(m)
    })
    return Array.from(existMonth).sort((a, b) => a - b)
  })

  const gridCellList = computed<GridCell[]>(() => {
    const cells: GridCell[] = Array.from({ length: TOTAL_CELL }, () => ({
      cellIndex: 0,
      date: null,
      record: null
    }))
    cells.forEach((item, idx) => {
      item.cellIndex = idx
    })

    const sortedRecords = [...ratingList.value].sort((a, b) =>
      new Date(b.date).getTime() - new Date(a.date).getTime()
    )

    const fillCount = Math.min(sortedRecords.length, TOTAL_CELL)
    for (let i = 0; i < fillCount; i++) {
      const cellIdx = TOTAL_CELL - 1 - i
      cells[cellIdx].date = sortedRecords[i].date
      cells[cellIdx].record = sortedRecords[i]
    }
    return cells
  })

  const dotTotalList = computed(() => Array.from({ length: dotCount }, (_, i) => i))
  const dotGridStyle = computed(() => ({
    gridTemplateColumns: `repeat(${DOT_COL}, 1fr)`
  }))

  return {
    totalRecordDays,
    totalRecordCount,
    hasOverflowRecord,
    fullDisplayMonthList,
    displayMonthList,
    gridCellList,
    dotTotalList,
    dotGridStyle
  }
}