# Ember - 个人效能追踪系统

> 基于 Tauri + Vue3 + Rust 的桌面端应用，用于追踪每日任务完成情况、能量-价值评分，并以热力图形式可视化展示。

---

## 📁 项目结构

```
Ember/
├── ember_gui/                    # 前端 + Tauri 后端
│   ├── src/                      # 前端 Vue 源码
│   │   ├── components/rating/    # 评分模块组件
│   │   ├── composables/          # Vue 组合式函数
│   │   ├── types/                # TypeScript 类型定义
│   │   ├── views/                # 页面视图
│   │   ├── App.vue               # 根组件
│   │   └── main.ts               # 入口文件
│   ├── src-tauri/                # Tauri Rust 后端
│   │   ├── src/
│   │   │   ├── commands/         # Tauri 命令处理器
│   │   │   ├── models/           # 数据模型
│   │   │   ├── services/         # 业务逻辑服务
│   │   │   ├── storage/          # 数据持久化
│   │   │   ├── lib.rs            # 库入口
│   │   │   └── main.rs           # 程序入口
│   │   └── Cargo.toml            # Rust 依赖配置
│   ├── package.json              # Node 依赖配置
│   └── index.html
└── README.md                     # 本文档
```

---

## 🏗️ 后端架构 (Rust)

### 1. 数据模型 (`src-tauri/src/models/`)

#### Task - 任务模型
```rust
pub struct Task {
    pub id: u64,                          // 任务唯一ID
    pub title: String,                    // 任务标题
    pub estimated_time: f32,              // 预估耗时（小时）
    pub estimated_difficulty: u8,         // 预估难度 1-5
    pub actual_time: Option<f32>,         // 实际耗时
    pub actual_satisfaction: Option<u8>,  // 实际满意度 1-5
    pub is_completed: bool,               // 是否完成
    pub completion_date: Option<NaiveDate>, // 完成日期
    pub is_important: bool,               // 是否为MIT（最重要任务）
    pub emoji_mood: Option<String>,       // 心情表情 😊😐😫🥳
    pub energy_score: Option<u8>,         // 精力消耗分 1-5（纵轴）
    pub value_score: Option<u8>,          // 贡献价值分 1-5（横轴）
    pub comment: Option<String>,          // 任务备注（≤20字）
    pub created_at: NaiveDate,            // 创建日期
}
```

#### DailyRating - 每日评分模型
```rust
pub struct DailyRating {
    pub date: NaiveDate,

    // === 用户直接填写的核心字段 ===
    pub work_type: String,          // 工作状态类型: "green"/"blue"/"orange"/"gray"
    pub energy_score: u8,           // 精力消耗分 1-5（纵轴）
    pub value_score: u8,            // 贡献价值分 1-5（横轴）
    pub work_hours: f32,            // 有效专注时长（小时）
    pub mood_emoji: String,         // 心情表情: "😊"/"😐"/"😫"/"🥳"
    pub comment: Option<String>,    // 每日评语（可选，≤50字）

    // === 后端计算生成的字段 ===
    pub productivity_color: String, // 最终RGBA颜色字符串
    pub achievement_score: u8,      // 综合成就分 0-100

    // === 扩展字段（自动统计）===
    pub status: u8,                 // 0-5 兼容旧版
    pub mit_completion_rate: f32,   // MIT任务完成率
    pub task_count_delta: i8,       // 与昨日任务量差值
    pub has_overdue: bool,          // 是否有逾期任务
    pub has_milestone: bool,        // 是否达成里程碑
}
```

#### AppState - 应用状态
```rust
pub struct AppState {
    pub tasks: Vec<Task>,                                    // 所有任务
    pub daily_ratings: HashMap<NaiveDate, DailyRating>,     // 每日评分映射
    pub next_task_id: u64,                                   // 下一个任务ID
}
```

---

### 2. Tauri 命令接口 (`src-tauri/src/commands/`)

#### 任务相关命令 (`task_commands.rs`)

| 命令名 | 参数 | 返回值 | 说明 |
|--------|------|--------|------|
| `create_task` | `title: String`<br>`estimated_time: f32`<br>`estimated_difficulty: u8`<br>`is_important: bool` | `Result<Task, String>` | 创建新任务 |
| `complete_task` | `task_id: u64`<br>`actual_time: f32`<br>`actual_satisfaction: u8`<br>`emoji_mood: String`<br>`energy_score: u8`<br>`value_score: u8`<br>`comment: Option<String>` | `Result<(), String>` | 完成任务并记录评分 |
| `get_tasks_by_date` | `date_str: String` | `Result<Vec<Task>, String>` | 获取指定日期的任务列表 |

#### 评分相关命令 (`rating_commands.rs`)

| 命令名 | 参数 | 返回值 | 说明 |
|--------|------|--------|------|
| `save_daily_rating` | `date_str: String`<br>`work_type: String`<br>`energy_score: u8`<br>`value_score: u8`<br>`work_hours: f32`<br>`mood_emoji: String`<br>`comment: Option<String>` | `Result<DailyRating, String>` | **保存每日评分（完整版）**，根据 work_type + energy/value 计算颜色 |
| `get_daily_rating` | `date_str: String` | `Result<DailyRating, String>` | 获取指定日期的评分 |
| `get_rating_list` | `start_date: String`<br>`end_date: String` | `Result<Vec<DailyRating>, String>` | 获取日期范围内的评分列表（热力图用） |
| `get_day_detail` | `date_str: String` | `Result<{rating, tasks}, String>` | 获取某天完整详情（评分 + 任务列表） |
| `delete_daily_rating` | `date_str: String` | `Result<(), String>` | 删除指定日期的评分 |

#### 统计相关命令 (`stats_commands.rs`)

| 命令名 | 参数 | 返回值 | 说明 |
|--------|------|--------|------|
| `get_heatmap_data` | `start_date: String`<br>`end_date: String`<br>`data_type: String` | `Result<serde_json::Value, String>` | 获取热力图数据 |

---

### 3. 服务层 (`src-tauri/src/services/`)

#### calc.rs - 计算工具
```rust
// 计算任务预估与实际偏差
pub fn calculate_deviation(task: &Task) -> Option<f32>

// 根据偏差值返回颜色标识
pub fn deviation_color(deviation: f32, threshold: f32) -> &'static str
```

#### stats.rs - 统计服务
```rust
// 根据任务完成情况自动计算并更新某天的 DailyRating
pub fn update_daily_rating(state: &mut AppState, date: NaiveDate)

// 合并自动计算和手动输入的数据
pub fn merge_daily_rating(auto_rating: &DailyRating, manual_rating: DailyRating) -> DailyRating
```

---

### 4. 数据持久化 (`src-tauri/src/storage/`)

```rust
// 获取数据文件路径（系统数据目录）
pub fn get_state_path() -> PathBuf

// 加载应用状态
pub fn load_state() -> AppState

// 保存应用状态
pub fn save_state(state: &AppState) -> Result<(), String>
```

**存储位置**: 
- Windows: `%LOCALAPPDATA%\ember_app\ember_data.json`
- macOS: `~/Library/Application Support/ember_app/ember_data.json`
- Linux: `~/.local/share/ember_app/ember_data.json`

---

## 🎨 热力图颜色规则

### 色相（工作状态类型 - 用户选择）
| 类型 | RGB值 | 含义 |
|------|-------|------|
| green | (34, 197, 94) | 绿色系 - 高产日 |
| blue | (59, 130, 246) | 蓝色系 - 学习日 |
| orange | (251, 146, 60) | 橙色系 - 忙碌但低效日 |
| gray | (136, 136, 136) | 灰色系 - 休息/摸鱼日 |

### 饱和度（由 energy/value 效率计算）
```rust
// 效率指数 = value_score / energy_score
// energy_score: 1-5 (1=低消耗，5=高消耗)
// value_score: 1-5 (1=低价值，5=高价值)
let efficiency = value_score / energy_score;

// 饱和度范围 0.3 - 1.0
let saturation = 0.3 + (efficiency.min(2.0) / 2.0) * 0.7;

// 最终颜色
productivity_color = "rgba(r, g, b, saturation)"
```

**效果：**
- **深色（高饱和度）**: 高价值 + 低消耗 = 高效率（黄金区）
- **浅色（低饱和度）**: 低价值 + 高消耗 = 低效率（垃圾区）

---

## 🎨 前端架构 (Vue3 + TypeScript)

### 组件结构

```
src/components/rating/
├── RatingHeader.vue        # 页面头部（统计信息、配置按钮）
├── RatingTabBar.vue        # 标签切换栏（评分/成长）
├── RatingHeatmap.vue       # 热力图网格展示
├── RatingGrowthPanel.vue   # 成长面板
├── RatingFooterCard.vue    # 底部统计卡片
└── RatingInputModal.vue    # 每日评分输入弹窗（待实现）
```

### 类型定义 (`src/types/rating.ts`)

```typescript
interface DailyRating {
  date: string
  work_type: string              // "green" | "blue" | "orange" | "gray"
  energy_score: number           // 1-5
  value_score: number            // 1-5
  work_hours: number
  mood_emoji: string             // "😊" | "😐" | "😫" | "🥳"
  comment: string | null
  productivity_color: string     // 热力图单元格背景色
  achievement_score: number      // 0-100
  status: number
  mit_completion_rate: number
  task_count_delta: number
  has_overdue: boolean
  has_milestone: boolean
}

interface GridCell {
  cellIndex: number
  date: string | null
  record: DailyRating | null
}
```

---

## 🔌 前后端通信

### 前端调用示例

```typescript
import { invoke } from '@tauri-apps/api/core'

// 1. 保存每日评分（核心功能）
const rating = await invoke<DailyRating>("save_daily_rating", {
  dateStr: "2025-07-27",
  workType: "green",        // 色相选择
  energyScore: 2,           // 精力消耗 1-5（纵轴）
  valueScore: 5,            // 贡献价值 1-5（横轴）
  workHours: 8.5,           // 工作时长
  moodEmoji: "😊",          // 心情表情
  comment: "今天效率很高，完成了重要任务"
})
// 返回的 rating.productivity_color 可直接用于热力图显示

// 2. 获取单日评分
const rating = await invoke<DailyRating>("get_daily_rating", {
  dateStr: "2025-07-27"
})

// 3. 获取日期范围评分（热力图用）
const ratings = await invoke<DailyRating[]>("get_rating_list", {
  startDate: "2025-07-01",
  endDate: "2025-07-31"
})

// 4. 获取某天详情（右侧面板用）
const detail = await invoke("get_day_detail", {
  dateStr: "2025-07-27"
})
// 返回: { rating: DailyRating, tasks: Task[] }

// 5. 删除评分
await invoke("delete_daily_rating", { dateStr: "2025-07-27" })

// 6. 创建任务
const task = await invoke<Task>("create_task", {
  title: "完成项目文档",
  estimatedTime: 2.5,
  estimatedDifficulty: 3,
  isImportant: true
})

// 7. 完成任务
await invoke("complete_task", {
  taskId: 1,
  actualTime: 3.0,
  actualSatisfaction: 4,
  emojiMood: "😊",
  energyScore: 3,
  valueScore: 5,
  comment: "质量很好"
})
```

---

## 🚀 开发命令

```bash
# 进入项目目录
cd ember_gui

# 安装依赖
npm install

# 开发模式（启动 Vite + Tauri）
npm run tauri dev

# 构建生产版本
npm run tauri build

# 仅前端开发
npm run dev
```

---

## 📋 功能实现状态

### 后端已实现 ✅
- [x] 数据模型（Task, DailyRating, AppState）
- [x] 热力图颜色计算（色相 + 饱和度）
- [x] 保存每日评分（`save_daily_rating`）
- [x] 查询评分（单日/范围/详情）
- [x] 删除评分
- [x] 任务创建与完成
- [x] 自动统计服务（`update_daily_rating`）
- [x] 数据本地持久化

### 前端已实现 ✅
- [x] 更新 TypeScript 类型定义（DailyRating, Task, DayDetail, SaveRatingForm）
- [x] 修改 API 调用（适配后端命令）
- [x] 每日评分输入弹窗（RatingInputModal.vue）
  - Emoji 心情选择（😊😐😫🥳）
  - 工作状态选择（绿/蓝/橙/灰）
  - 二维评分图表（能量-价值坐标）
  - 工作时长输入
  - 评语输入
- [x] 右侧面板（DayDetailPanel.vue）
  - 显示当天评分详情
  - 显示任务清单
  - 编辑/添加评分按钮
- [x] 热力图点击交互

### 前端待优化 🚧
- [ ] 悬停浮窗展示详细信息（当前只有简单的 tooltip）
- [ ] 响应式布局优化
- [ ] 加载状态优化

---

## 📝 数据文件示例

```json
{
  "tasks": [
    {
      "id": 1,
      "title": "完成项目文档",
      "estimated_time": 2.5,
      "estimated_difficulty": 3,
      "actual_time": 3.0,
      "actual_satisfaction": 4,
      "is_completed": true,
      "completion_date": "2025-07-27",
      "is_important": true,
      "emoji_mood": "😊",
      "energy_score": 3,
      "value_score": 5,
      "comment": "质量很好",
      "created_at": "2025-07-27"
    }
  ],
  "daily_ratings": {
    "2025-07-27": {
      "date": "2025-07-27",
      "work_type": "green",
      "energy_score": 2,
      "value_score": 5,
      "work_hours": 8.5,
      "mood_emoji": "😊",
      "comment": "今天效率很高",
      "productivity_color": "rgba(34, 197, 94, 1.00)",
      "achievement_score": 100,
      "status": 0,
      "mit_completion_rate": 1.0,
      "task_count_delta": 2,
      "has_overdue": false,
      "has_milestone": false
    }
  },
  "next_task_id": 2
}
```

---

## 🔧 技术栈

- **后端**: Rust + Tauri v2
- **前端**: Vue 3 + TypeScript + Vite
- **状态管理**: 原生 Tauri State (Mutex<AppState>)
- **数据存储**: JSON 文件本地存储
- **日期处理**: chrono (Rust) / 原生 Date (TS)
- **序列化**: serde + serde_json

---

## 📄 License

MIT