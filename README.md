# Tauri + Vue + TypeScript

This template should help get you started developing with Vue 3 and TypeScript in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

Rust结构如下：
src-tauri/src/
├── lib.rs                 # Tauri 应用入口，注册命令和状态
├── models/                # 数据模型定义
│   ├── mod.rs
│   ├── task.rs            # 任务结构体
│   ├── daily_rating.rs    # 每日评分（情绪、能量、价值）
│   ├── milestone.rs       # 里程碑事件（可选）
│   └── app_state.rs       # 全局应用状态（包含所有数据）
├── storage/               # 数据持久化（读写文件）
│   ├── mod.rs
│   └── file_store.rs      # 序列化/反序列化到 JSON 或 RON
├── services/              # 核心业务逻辑
│   ├── mod.rs
│   ├── calc.rs            # 偏差计算、评分聚合
│   └── stats.rs           # 统计函数（如任务量变化、趋势）
└── commands/              # Tauri 命令（暴露给前端）
    ├── mod.rs
    ├── task_commands.rs   # 创建、完成、查询任务
    ├── rating_commands.rs # 保存每日评分
    └── stats_commands.rs  # 获取热力图数据、统计摘要

DailyRating —— 每日评分记录
字段	类型	说明
date	string	日期，格式 "YYYY-MM-DD"
status	number	0~5，对应五档评价
comment	string | null	短评，最多20字
mood_emoji	string	表情符号，如 "🟢"、"❌"
productivity_color	string	CSS 颜色值，如 "rgba(74, 222, 128, 0.85)"
work_hours	number	工作时长（小时），暂未启用
achievement_score	number	综合成就分 0~100，暂未启用
mit_completion_rate	number	MIT完成率 0~1，暂未启用
task_count_delta	number	任务量同比变化，暂未启用
has_overdue	boolean	是否有逾期，暂未启用
has_milestone	boolean	是否有里程碑，暂未启用


Task —— 任务模型（暂未使用）
字段	类型	说明
id	number	任务ID
title	string	任务标题
estimated_time	number	预估耗时（小时）
estimated_difficulty	number	预估难度 1~5
actual_time	number | null	实际耗时
actual_satisfaction	number | null	实际满意度 1~5
is_completed	boolean	是否完成
completion_date	string | null	完成日期
is_important	boolean	是否为 MIT
emoji_mood	string | null	心情表情
energy_score	number | null	精力评分 1~5
value_score	number | null	价值评分 1~5
comment	string | null	任务评语
created_at	string	创建日期