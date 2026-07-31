import { ref, computed, onMounted, onUnmounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { playStartupAudio, playTimeAudio } from './usePetAudio'

export type PetState = 'stand' | 'walk-left' | 'walk-right'

export interface PetPosition {
  x: number
  y: number
}

// 宠物配置
const PET_CONFIG = {
  width: 250,
  height: 250,
  speed: 2,
  animationInterval: 400,
  behaviorMinTime: 5000,
  behaviorMaxTime: 12000,
}

export function usePet() {
  // 状态
  const state = ref<PetState>('stand')
  const position = ref<PetPosition>({ x: 0, y: 0 })
  const direction = ref<'left' | 'right'>('right')
  const isDragging = ref(false)
  const currentFrameIndex = ref(0)
  const screenSize = ref({ width: 1920, height: 1080 })

  // 动画帧缓存
  const frames = ref<Record<PetState, string[]>>({
    stand: [],
    'walk-left': [],
    'walk-right': [],
  })

  // 当前帧
  const currentFrame = computed(() => {
    const stateFrames = frames.value[state.value]
    if (stateFrames.length === 0) return null
    return stateFrames[currentFrameIndex.value % stateFrames.length]
  })

  // 是否使用占位符
  const usePlaceholder = computed(() => {
    return frames.value[state.value].length === 0
  })

  // 加载帧图片（从 public/pet/ 加载）
  const loadFrames = async () => {
    const configs = [
      { state: 'stand' as PetState, folder: 'stand', count: 6 },
      { state: 'walk-left' as PetState, folder: 'walk-left', count: 7 },
      { state: 'walk-right' as PetState, folder: 'walk-right', count: 7 },
    ]

    for (const config of configs) {
      const loadedFrames: string[] = []
      for (let i = 1; i <= config.count; i++) {
        // 使用 public 目录下的路径（开发和打包模式通用）
        const path = `/pet/${config.folder}/${i}.png`
        
        try {
          const response = await fetch(path, { method: 'HEAD' })
          if (response.ok) {
            loadedFrames.push(path)
          }
        } catch {
          // 图片不存在，跳过
        }
      }
      if (loadedFrames.length > 0) {
        frames.value[config.state] = loadedFrames
        console.log(`[Pet] Loaded ${loadedFrames.length} frames for ${config.state}`)
      }
    }
  }

  // 获取屏幕尺寸
  const getScreenSize = async () => {
    try {
      const [width, height] = await invoke<[number, number]>('get_screen_size')
      screenSize.value = { width, height }
      console.log('[Pet] Screen size:', width, height)
      console.log('[Pet] Pet width:', PET_CONFIG.width)
      console.log('[Pet] Max X should be:', width - PET_CONFIG.width)
    } catch (e) {
      console.log('[Pet] Failed to get screen size:', e)
    }
  }

  // 获取窗口位置
  const getWindowPosition = async (): Promise<{ x: number; y: number }> => {
    try {
      const [x, y] = await invoke<[number, number]>('get_pet_window_position')
      return { x, y }
    } catch (e) {
      console.log('[Pet] Failed to get window position:', e)
      return { x: 0, y: 0 }
    }
  }

  // 移动窗口
  const moveWindow = async (x: number, y: number) => {
    try {
      await invoke('move_pet_window', { x, y })
      position.value = { x, y }
    } catch (e) {
      console.log('[Pet] Failed to move window:', e)
    }
  }

  // 播放启动音频
  const playStartup = async () => {
    console.log('[Pet] Playing startup audio')
    await playStartupAudio()
  }

  // 播放时段音频
  const playPeriodAudio = async () => {
    console.log('[Pet] Playing period audio')
    await playTimeAudio()
  }

  // 定时器
  let animationTimer: ReturnType<typeof setInterval> | null = null
  let moveTimer: ReturnType<typeof setInterval> | null = null
  let behaviorTimer: ReturnType<typeof setTimeout> | null = null
  let audioTimer: ReturnType<typeof setTimeout> | null = null

  // 开始动画
  const startAnimation = () => {
    if (animationTimer) clearInterval(animationTimer)
    animationTimer = setInterval(() => {
      const stateFrames = frames.value[state.value]
      if (stateFrames.length > 0) {
        currentFrameIndex.value = (currentFrameIndex.value + 1) % stateFrames.length
      }
    }, PET_CONFIG.animationInterval)
  }

  // 停止动画
  const stopAnimation = () => {
    if (animationTimer) {
      clearInterval(animationTimer)
      animationTimer = null
    }
  }

  // 停止移动
  const stopMoving = () => {
    if (moveTimer) {
      clearInterval(moveTimer)
      moveTimer = null
    }
  }

  // 开始走动
  const startWalking = (dir: number) => {
    stopMoving()
    moveTimer = setInterval(async () => {
      if (isDragging.value) return

      const step = dir * PET_CONFIG.speed
      const currentPos = await getWindowPosition()
      const newX = currentPos.x + step

      // 检测屏幕边界
      const maxX = screenSize.value.width - PET_CONFIG.width
      const minX = 0

      console.log('[Pet] Current pos:', currentPos.x, 'New X:', newX, 'Max X:', maxX)

      if (newX <= minX) {
        // 碰到左边界，切换到向右走
        console.log('[Pet] Hit left boundary!')
        await moveWindow(minX, currentPos.y)
        if (state.value === 'walk-left') {
          setState('walk-right')
        }
      } else if (newX >= maxX) {
        // 碰到右边界，切换到向左走
        console.log('[Pet] Hit right boundary!')
        await moveWindow(maxX, currentPos.y)
        if (state.value === 'walk-right') {
          setState('walk-left')
        }
      } else {
        await moveWindow(newX, currentPos.y)
      }
    }, 30)
  }

  // 设置状态
  const setState = (newState: PetState) => {
    if (state.value === newState) return

    state.value = newState
    currentFrameIndex.value = 0

    if (newState === 'walk-left') {
      direction.value = 'left'
      startWalking(-1)
    } else if (newState === 'walk-right') {
      direction.value = 'right'
      startWalking(1)
    } else {
      stopMoving()
    }
  }

  // 随机行为
  const randomBehavior = () => {
    if (isDragging.value) {
      scheduleNextBehavior()
      return
    }

    const behaviors: PetState[] = ['stand', 'walk-left', 'walk-right']
    const weights = [0.4, 0.3, 0.3]

    const random = Math.random()
    let cumulativeWeight = 0
    let selectedBehavior: PetState = 'stand'

    for (let i = 0; i < behaviors.length; i++) {
      cumulativeWeight += weights[i]
      if (random < cumulativeWeight) {
        selectedBehavior = behaviors[i]
        break
      }
    }

    setState(selectedBehavior)
    scheduleNextBehavior()
  }

  // 安排下次行为
  const scheduleNextBehavior = () => {
    if (behaviorTimer) clearTimeout(behaviorTimer)
    const nextTime = PET_CONFIG.behaviorMinTime + Math.random() * (PET_CONFIG.behaviorMaxTime - PET_CONFIG.behaviorMinTime)
    behaviorTimer = setTimeout(randomBehavior, nextTime)
  }

  // 拖拽处理
  const startDrag = async (e: MouseEvent) => {
    e.preventDefault()
    isDragging.value = true
    
    const startPos = await getWindowPosition()
    const startX = e.screenX - startPos.x
    const startY = e.screenY - startPos.y

    const onMouseMove = async (ev: MouseEvent) => {
      const newX = ev.screenX - startX
      const newY = ev.screenY - startY
      await moveWindow(newX, newY)
    }

    const onMouseUp = () => {
      isDragging.value = false
      window.removeEventListener('mousemove', onMouseMove)
      window.removeEventListener('mouseup', onMouseUp)
      scheduleNextBehavior()
    }

    window.addEventListener('mousemove', onMouseMove)
    window.addEventListener('mouseup', onMouseUp)
  }

  // 时段音频循环（5-20分钟随机间隔）
  const schedulePeriodAudio = () => {
    playPeriodAudio()
    const nextTime = (5 + Math.random() * 15) * 60 * 1000 // 5-20分钟随机
    audioTimer = setTimeout(schedulePeriodAudio, nextTime)
  }

  // 启动
  const init = async () => {
    await getScreenSize()
    await loadFrames()
    startAnimation()
    randomBehavior()

    // 延迟播放启动音频（等用户交互）
    setTimeout(() => {
      playStartup()
    }, 1000)

    // 启动时段音频（首次20-40秒后播放）
    const firstDelay = (20 + Math.random() * 20) * 1000 // 20-40秒随机
    audioTimer = setTimeout(schedulePeriodAudio, firstDelay)
  }

  // 清理
  const destroy = () => {
    stopAnimation()
    stopMoving()
    if (behaviorTimer) clearTimeout(behaviorTimer)
    if (audioTimer) clearTimeout(audioTimer)
  }

  onMounted(() => {
    init()
  })

  onUnmounted(() => {
    destroy()
  })

  return {
    state,
    position,
    direction,
    isDragging,
    currentFrame,
    usePlaceholder,
    startDrag,
  }
}