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
    pub date: NaiveDate,                  // 日期
    pub status: u8,                       // 状态等级 0-5
    pub productivity_color: String,       // 生产力颜色（RGBA字符串）
    pub work_hours: f32,                  // 有效专注时长
    pub comment: Option<String>,          // 每日评语
    pub mood_emoji: String,               // 心情表情
    pub achievement_score: u8,            // 成就分 0-100
    pub mit_completion_rate: f32,         // MIT完成率
    pub task_count_delta: i8,             // 与昨日任务量差值
    pub has_overdue: bool,                // 是否有逾期任务
    pub has_milestone: bool,              // 是否有里程碑
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
| `save_daily_rating` | `date_str: String`<br>`status: u8`<br>`comment: Option<String>` | `Result<(), String>` | 保存每日评分（基础版） |
| `get_daily_rating` | `date_str: String` | `Result<DailyRating, String>` | 获取指定日期的评分 |

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
// 更新某天的综合评分（根据当天完成任务自动计算）
pub fn update_daily_rating(state: &mut AppState, date: NaiveDate)
```

**当前实现状态**: `update_daily_rating` 函数目前返回硬编码的默认值，需要根据实际业务逻辑完善：
- 根据任务完成情况计算工作时长
- 根据 energy_score/value_score 计算热力图颜色饱和度
- 统计 MIT 完成率
- 计算成就分数

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

## 🎨 前端架构 (Vue3 + TypeScript)

### 组件结构

```
src/components/rating/
├── RatingHeader.vue        # 页面头部（统计信息、配置按钮）
├── RatingTabBar.vue        # 标签切换栏（评分/成长）
├── RatingHeatmap.vue       # 热力图网格展示
├── RatingGrowthPanel.vue   # 成长面板（待实现）
└── RatingFooterCard.vue    # 底部统计卡片
```

### 组合式函数

#### useRatingApi.ts
```typescript
// 获取评分列表和主题配置
const fetchAllData: () => Promise<void>

// 返回的数据
{
  ratingList: DailyRating[],      // 评分列表
  globalConfig: GlobalThemeConfig, // 主题配置
  continueDays: number,           // 连续记录天数
  loading: boolean,
  loadError: boolean
}
```

#### useRatingHeatmap.ts
```typescript
// 热力图数据处理
{
  totalRecordDays: number,        // 总记录天数
  totalRecordCount: number,       // 总记录数
  hasOverflowRecord: boolean,     // 是否超出显示限制
  fullDisplayMonthList: number[], // 显示的月份列表
  gridCellList: GridCell[],       // 网格单元格数据
  dotTotalList: number[],         // 点阵统计数据
  dotGridStyle: object            // 点阵样式
}
```

### 类型定义 (`src/types/rating.ts`)

```typescript
interface DailyRating {
  date: string
  status: number
  comment: string | null
  mood_emoji: string
  productivity_color: string    // 热力图单元格背景色
  work_hours: number
  achievement_score: number
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

// 创建任务
const task = await invoke<Task>("create_task", {
  title: "完成项目文档",
  estimatedTime: 2.5,
  estimatedDifficulty: 3,
  isImportant: true
})

// 完成任务（带评分）
await invoke("complete_task", {
  taskId: 1,
  actualTime: 3.0,
  actualSatisfaction: 4,
  emojiMood: "😊",
  energyScore: 3,
  valueScore: 5,
  comment: "比预期耗时久但质量很好"
})

// 获取某日评分
const rating = await invoke<DailyRating>("get_daily_rating", {
  dateStr: "2025-07-27"
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

### 已实现 ✅
- [x] 基础任务创建与完成
- [x] 任务字段支持 energy_score/value_score/emoji_mood
- [x] 每日评分基础数据结构
- [x] 热力图网格展示
- [x] 数据本地持久化
- [x] 基础 Tauri 命令接口

### 待实现 🚧
- [ ] 完整的每日评分输入弹窗（Emoji选择、工作状态选择、二维评分图表）
- [ ] 根据二维评分计算热力图颜色饱和度（高价值+低消耗=深色）
- [ ] 右侧面板展示当天任务清单
- [ ] 悬停浮窗展示详细信息
- [ ] 完善 `update_daily_rating` 统计逻辑
- [ ] 新增/完善 Tauri 命令接口（见下方规划）

---

## 🔮 计划新增的 Tauri 命令

### 每日评分完整版
```rust
#[command]
pub fn save_daily_rating_full(
    date: String,
    work_type: String,        // "green" | "blue" | "orange" | "gray"
    energy_score: u8,         // 1-5 精力消耗
    value_score: u8,          // 1-5 贡献价值
    work_hours: f32,
    mood_emoji: String,       // "😊" | "😐" | "😫" | "🥳"
    comment: Option<String>,
) -> Result<DailyRating, String>
```

### 获取某天完整数据（右侧面板用）
```rust
#[command]
pub fn get_day_detail(
    date: String,
) -> Result<DayDetail, String>

struct DayDetail {
    rating: DailyRating,
    tasks: Vec<Task>,
}
```

### 获取日期范围评分列表
```rust
#[command]
pub fn get_rating_list_by_range(
    start_date: String,
    end_date: String,
) -> Result<Vec<DailyRating>, String>
```

---

## 🎨 热力图颜色规则

### 色相（工作状态类型）
| 类型 | 颜色 | 含义 |
|------|------|------|
| green | 绿色系 | 高产日 |
| blue | 蓝色系 | 学习日 |
| orange | 橙色系 | 忙碌但低效日 |
| gray | 灰色系 | 休息/摸鱼日 |

### 饱和度（投入程度）
由二维评分计算得出：
- **深色（高饱和度）**: 高价值 + 低消耗（黄金区）
- **浅色（低饱和度）**: 低价值 + 高消耗（垃圾区）

计算公式（规划中）：
```rust
// 效率指数 = value_score / energy_score
// 饱和度 = f(效率指数, work_hours)
```

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
      "status": 5,
      "productivity_color": "rgba(34, 197, 94, 0.85)",
      "work_hours": 8.5,
      "comment": "今天效率很高",
      "mood_emoji": "😊",
      "achievement_score": 85,
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