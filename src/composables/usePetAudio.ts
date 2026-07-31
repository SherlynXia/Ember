import { invoke } from '@tauri-apps/api/core'

/**
 * 播放启动音频（随机选择）
 */
export async function playStartupAudio(): Promise<void> {
  try {
    console.log('[Pet] Playing startup audio via Rust')
    await invoke('play_random_startup_audio')
  } catch (error) {
    console.error('[Pet] Failed to play startup audio:', error)
  }
}

/**
 * 播放时段音频（根据当前时间）
 */
export async function playTimeAudio(): Promise<void> {
  try {
    console.log('[Pet] Playing time audio via Rust')
    await invoke('play_random_time_audio')
  } catch (error) {
    console.error('[Pet] Failed to play time audio:', error)
  }
}

/**
 * 播放指定路径的音频
 */
export async function playAudioFile(path: string): Promise<void> {
  try {
    console.log('[Pet] Playing audio:', path)
    await invoke('play_audio', { path })
  } catch (error) {
    console.error('[Pet] Failed to play audio:', error)
  }
}