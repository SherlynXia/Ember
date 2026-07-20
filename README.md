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