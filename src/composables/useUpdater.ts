import { invoke } from '@tauri-apps/api/core'
import { ref } from 'vue'

// 弹窗状态
export const dialogVisible = ref(false)
export const dialogTitle = ref('')
export const dialogMessage = ref('')
export const dialogShowCancel = ref(false)

let dialogResolve: ((value: boolean) => void) | null = null

// 显示弹窗
export function showDialog(title: string, message: string, showCancel: boolean = false): Promise<boolean> {
  return new Promise((resolve) => {
    dialogTitle.value = title
    dialogMessage.value = message
    dialogShowCancel.value = showCancel
    dialogVisible.value = true
    dialogResolve = resolve
  })
}

// 确认弹窗
export function confirmDialog() {
  dialogVisible.value = false
  if (dialogResolve) {
    dialogResolve(true)
    dialogResolve = null
  }
}

// 取消弹窗
export function cancelDialog() {
  dialogVisible.value = false
  if (dialogResolve) {
    dialogResolve(false)
    dialogResolve = null
  }
}

export function useUpdater() {
  // 检查更新
  async function checkForUpdates() {
    try {
      console.log('正在检查更新...')
      const result = await invoke<string>('check_update')
      console.log('更新检查结果:', result)

      if (result.includes('发现新版本')) {
        const yes = await showDialog(
          '发现新版本',
          `${result}，是否立即更新？`,
          true
        )

        if (yes) {
          // 下载并安装更新
          await invoke('download_and_install_update')
        }
      } else {
        await showDialog('检查更新', result)
      }
    } catch (error) {
      console.error('检查更新失败:', error)
      const errorMessage = String(error)
      
      // 如果包含 GitHub 地址，提供打开浏览器的选项
      if (errorMessage.includes('github.com')) {
        const yes = await showDialog(
          '检查更新失败',
          `${errorMessage}\n\n是否打开浏览器手动检查？`,
          true
        )
        
        if (yes) {
          // 使用 Tauri 命令打开浏览器
          await invoke('open_browser', { url: 'https://github.com/SherlynXia/Ember/releases' })
        }
      } else {
        await showDialog('错误', `检查更新失败: ${error}`)
      }
    }
  }

  // 静默检查（启动时自动检查）
  async function checkForUpdatesSilent() {
    try {
      const result = await invoke<string>('check_update')
      console.log('静默更新检查结果:', result)

      if (result.includes('发现新版本')) {
        const yes = await showDialog(
          '发现新版本',
          `${result}，是否立即更新？`,
          true
        )

        if (yes) {
          await invoke('download_and_install_update')
        }
      }
    } catch (error) {
      // 静默失败，不打扰用户
      console.error('静默检查更新失败:', error)
    }
  }

  return {
    checkForUpdates,
    checkForUpdatesSilent,
  }
}