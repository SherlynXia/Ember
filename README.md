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


src
├─ types/          
# 【全局类型定义】数据格式说明书、前后端交互契约
│  └─ rating.ts
# 评分模块全部TS类型定义：DailyRating单条记录、GlobalThemeConfig全局配色、GridCell热力图格子、RatingRes后端返回数据包
# 所有组件、composables统一从这里导入类型，TS校验数据结构，防止字段写错

├─ composables/    
# 【业务逻辑层】纯TS代码，无HTML、无样式，和界面完全解耦，逻辑可复用
│  ├─ useRatingApi.ts
# 负责与Tauri Rust后端通信，发起invoke请求
# 获取全局主题配色、获取全部评分原始数据；存放全局响应式状态（评分列表、配色、加载状态）
│  └─ useRatingHeatmap.ts
# 热力图专属计算逻辑
# 接收原始评分数组，自动派生：打卡总天数、月份范围、热力图格子数组等页面渲染需要的计算数据

├─ components/rating/  
# 【UI小组件，页面积木】只负责渲染界面+样式，不主动请求后端
# 所有数据、配置统一由父页面通过props传入，单一职责，一块界面对应一个文件
│  ├─ RatingHeader.vue
# 页面顶部头部区域：标题、打卡统计文字、加载提示、报错提示、打开配色面板按钮
│  ├─ RatingTabBar.vue
# 顶部标签切换栏：【每日评分 | 成长度】，控制页面视图切换
│  ├─ RatingHeatmap.vue
# 热力图主体区域：渲染日期色块、月份标尺、悬浮tooltip、格子点击事件
│  ├─ RatingGrowthPanel.vue
# 成长度视图面板（预留扩展），切换标签后展示，后续用来放置趋势图表
│  └─ RatingFooterCard.vue
# 页面底部卡片：说明文字、装饰圆点、底部配色设置按钮

├─ views/          
# 【完整页面容器】一整张独立页面，承上启下的调度中心
│  └─ RatingRecord.vue
# 评分记录页面主页面
# 1.引入composables拿到数据与计算结果
# 2.导入所有rating下的UI子组件
# 3.把数据、全局配色通过props下发给各个子组件
# 4.统一处理页面生命周期、各类点击事件、视图切换

├─ App.vue         
# 【项目顶层根组件】整个Vue应用最外层容器
# 当前单页面结构：直接载入并渲染RatingRecord页面
# 未来做多页面/弹窗时，可以在这里增加路由、全局弹窗管理

└─ main.ts         
# 【程序启动入口】整个前端最先执行的代码
# 创建Vue应用实例、挂载到DOM、初始化环境、衔接Tauri运行环境