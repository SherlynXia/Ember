import { invoke } from '@tauri-apps/api/core'
import type { Task } from '../types/rating'

// ==================== 基础 CRUD ====================

/** 创建任务 */
export async function createTask(
  title: string,
  estimatedTime: number,
  estimatedDifficulty: number,
  isImportant: boolean
): Promise<Task> {
  return await invoke('create_task', {
    title,
    estimatedTime,
    estimatedDifficulty,
    isImportant
  })
}

/** 更新任务 */
export async function updateTask(
  taskId: number,
  updates: {
    title?: string
    estimatedTime?: number
    estimatedDifficulty?: number
    isImportant?: boolean
    comment?: string
  }
): Promise<Task> {
  return await invoke('update_task', {
    taskId,
    ...updates
  })
}

/** 删除任务 */
export async function deleteTask(taskId: number): Promise<void> {
  return await invoke('delete_task', { taskId })
}

/** 完成任务 */
export async function completeTask(
  taskId: number,
  actualTime: number,
  actualSatisfaction: number,
  emojiMood: string,
  energyScore: number,
  valueScore: number,
  comment?: string
): Promise<void> {
  return await invoke('complete_task', {
    taskId,
    actualTime,
    actualSatisfaction,
    emojiMood,
    energyScore,
    valueScore,
    comment
  })
}

// ==================== 查询 ====================

/** 按日期获取任务 */
export async function getTasksByDate(dateStr: string): Promise<Task[]> {
  return await invoke('get_tasks_by_date', { dateStr })
}

/** 获取未分配的任务（待办池） */
export async function getUnassignedTasks(): Promise<Task[]> {
  return await invoke('get_unassigned_tasks')
}

/** 获取所有任务 */
export async function getAllTasks(): Promise<Task[]> {
  return await invoke('get_all_tasks')
}

/** 获取任务详情 */
export async function getTaskDetail(taskId: number): Promise<Task> {
  return await invoke('get_task_detail', { taskId })
}

// ==================== 拖拽和日期分配 ====================

/** 分配任务到日期（拖拽） */
export async function assignTaskToDate(
  taskId: number,
  dateStr: string | null
): Promise<Task> {
  return await invoke('assign_task_to_date', { taskId, dateStr })
}

/** 重新排序任务 */
export async function reorderTasks(taskIds: number[]): Promise<void> {
  return await invoke('reorder_tasks', { taskIds })
}

// ==================== 子任务 ====================

/** 创建子任务 */
export async function createSubTask(
  parentId: number,
  title: string,
  estimatedTime: number,
  estimatedDifficulty: number
): Promise<Task> {
  return await invoke('create_sub_task', {
    parentId,
    title,
    estimatedTime,
    estimatedDifficulty
  })
}

/** 获取子任务列表 */
export async function getSubTasks(parentId: number): Promise<Task[]> {
  return await invoke('get_sub_tasks', { parentId })
}

/** 提升子任务为独立任务 */
export async function promoteSubTask(taskId: number): Promise<Task> {
  return await invoke('promote_sub_task', { taskId })
}