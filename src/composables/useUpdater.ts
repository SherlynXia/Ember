import { check } from '@tauri-apps/plugin-updater'
import { ask, message } from '@tauri-apps/plugin-dialog'

export function useUpdater() {
  // 检查更新
  async function checkForUpdates() {
    try {
      const update = await check()

      if (update) {
        console.log('发现新版本:', update.version)

        const yes = await ask(
          `发现新版本 ${update.version}，是否立即更新？\n\n更新内容：${update.body || '暂无说明'}`,
          {
            title: '发现新版本',
            kind: 'info',
          }
        )

        if (yes) {
          await update.downloadAndInstall()
          // 下载完成后会提示重启
        }
      } else {
        console.log('当前已是最新版本')
      }
    } catch (error) {
      console.error('检查更新失败:', error)
    }
  }

  // 静默检查（启动时自动检查）
  async function checkForUpdatesSilent() {
    try {
      const update = await check()

      if (update) {
        console.log('发现新版本:', update.version)

        const yes = await ask(
          `发现新版本 ${update.version}，是否立即更新？`,
          {
            title: '发现新版本',
            kind: 'info',
          }
        )

        if (yes) {
          await update.downloadAndInstall()
        }
      }
    } catch (error) {
      // 静默失败，不打扰用户
      console.error('检查更新失败:', error)
    }
  }

  return {
    checkForUpdates,
    checkForUpdatesSilent,
  }
}