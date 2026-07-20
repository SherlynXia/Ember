<script setup lang="ts">
import { ref, reactive, computed, onMounted, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";

// 5档评价状态
type DayStatus = 0 | 1 | 2 | 3 | 4 | 5;

// 状态对应分数、颜色、阴影、emoji
const statusScoreMap: Record<DayStatus, number> = {
  0: 0, // 无记录
  1: 1, // 很差 红
  2: 2, // 一般 橙
  3: 3, // 普通 黄
  4: 4, // 良好 浅绿
  5: 5, // 优秀 深绿
};

const statusMap = [
  { color: "#888", shadow: "#666", emoji: "⬜", label: "无记录", score: 0 },
  { color: "#ef4444", shadow: "#b91c1c", emoji: "❌", label: "很差（红）", score: 1 },
  { color: "#fb923c", shadow: "#c2410c", emoji: "⚠️", label: "一般（橙）", score: 2 },
  { color: "#facc15", shadow: "#a16207", emoji: "🟡", label: "普通（黄）", score: 3 },
  { color: "#4ade80", shadow: "#16a34a", emoji: "🟢", label: "良好（浅绿）", score: 4 },
  { color: "#22c55e", shadow: "#15803d", emoji: "✅", label: "优秀（深绿）", score: 5 },
];

type MonthRecord = Record<number, { status: DayStatus; text: string }>;
type YearRecord = Record<number, MonthRecord>;

const allYearData = reactive<Record<number, YearRecord>>({});

const currentYear = ref(new Date().getFullYear());
const currentMonth = ref(new Date().getMonth() + 1);

const showPopup = ref(false);
const selectDay = ref(0);
const tempText = ref("");

// 闰年判断
const isLeapYear = computed(() => {
  const year = currentYear.value;
  return (year % 4 === 0 && year % 100 !== 0) || year % 400 === 0;
});

// 当月天数
const monthMaxDay = computed(() => {
  const m = currentMonth.value;
  if ([1, 3, 5, 7, 8, 10, 12].includes(m)) return 31;
  if ([4, 6, 9, 11].includes(m)) return 30;
  return isLeapYear.value ? 29 : 28;
});

const dayList = computed(() =>
  Array.from({ length: monthMaxDay.value }, (_, i) => i + 1)
);

// 获取单日数据
const getDayInfo = (day: number) => {
  const yearData = allYearData[currentYear.value] ?? {};
  const monthData = yearData[currentMonth.value] ?? {};
  return monthData[day] ?? { status: 0, text: "" };
};

// 截取前4字预览
const getShortText = (text: string) => {
  if (!text) return "";
  return text.slice(0, 4);
};

// 曲线数据源
const curveData = computed(() => {
  return dayList.value.map((day) => {
    const info = getDayInfo(day);
    return {
      day,
      score: statusScoreMap[info.status],
    };
  });
});

// 打开编辑弹窗
function openEdit(day: number) {
  selectDay.value = day;
  const info = getDayInfo(day);
  tempText.value = info.text;
  showPopup.value = true;
}

// 保存记录
function saveRecord(status: DayStatus) {
  const trimText = tempText.value.slice(0, 20);

  if (!allYearData[currentYear.value]) {
    allYearData[currentYear.value] = {};
  }
  if (!allYearData[currentYear.value][currentMonth.value]) {
    allYearData[currentYear.value][currentMonth.value] = {};
  }

  allYearData[currentYear.value][currentMonth.value][selectDay.value] = {
    status,
    text: trimText,
  };

  showPopup.value = false;
}

// 切换年月
function changeMonth(num: number) {
  currentMonth.value = num;
}
function addYear() {
  currentYear.value += 1;
}
function subYear() {
  currentYear.value -= 1;
}

// 【可选持久化】软件启动读取本地存档，需要配合之前给你的Rust commands
onMounted(async () => {
  try {
    const jsonStr = await invoke<string>("load_record");
    const saved = JSON.parse(jsonStr);
    Object.assign(allYearData, saved);
  } catch (err) {
    console.log("暂无本地存档", err);
  }
});
watch(allYearData, async (val) => {
  try {
    const jsonStr = JSON.stringify(val);
    await invoke("save_record", { data: jsonStr });
  } catch (e) {}
}, { deep: true });
</script>

<template>
  <main class="container">
    <div class="year-wrap">
      <h2>年度每日评价记录</h2>

      <div class="year-switch">
        <button @click="subYear">◀</button>
        <span class="year-text">{{ currentYear }} 年</span>
        <button @click="addYear">▶</button>
        <span class="leap-tip" v-if="currentMonth === 2">
          {{ isLeapYear ? "闰年29天" : "平年28天" }}
        </span>
      </div>

      <div class="month-tabs">
        <button
          v-for="m in 12"
          :key="m"
          class="month-btn"
          :class="{ active: currentMonth === m }"
          @click="changeMonth(m)"
        >
          {{ m }}月
        </button>
      </div>

      <!-- 5档图例 -->
      <div class="legend">
        <span v-for="item in statusMap" :key="item.label">
          <span class="color-block" :style="{ background: item.color }"></span>
          {{ item.emoji }} {{ item.label }}
        </span>
      </div>

      <div class="chart-box">
        <h4>{{ currentYear }}年{{ currentMonth }}月 状态趋势曲线</h4>
        <svg
          width="100%"
          height="160"
          viewBox="0 0 720 160"
          preserveAspectRatio="none"
        >
          <!-- 5档横向参考线 -->
          <line x1="0" y1="20" x2="720" y2="20" stroke="#888" stroke-width="0.5" opacity="0.4" />
          <line x1="0" y1="48" x2="720" y2="48" stroke="#888" stroke-width="0.5" opacity="0.4" />
          <line x1="0" y1="76" x2="720" y2="76" stroke="#888" stroke-width="0.5" opacity="0.4" />
          <line x1="0" y1="104" x2="720" y2="104" stroke="#888" stroke-width="0.5" opacity="0.4" />
          <line x1="0" y1="132" x2="720" y2="132" stroke="#888" stroke-width="0.5" opacity="0.4" />
          <!-- 等级文字 -->
          <text x="8" y="18" font-size="11" fill="#aaa">优秀</text>
          <text x="8" y="46" font-size="11" fill="#aaa">良好</text>
          <text x="8" y="74" font-size="11" fill="#aaa">普通</text>
          <text x="8" y="102" font-size="11" fill="#aaa">一般</text>
          <text x="8" y="130" font-size="11" fill="#aaa">很差</text>
          <path
            :d="curveData.reduce((path, point, idx) => {
              const totalWidth = 700;
              const step = totalWidth / (dayList.length - 1 || 1);
              const x = 10 + idx * step;
              const y = 140 - point.score * 24;
              return idx === 0 ? `M ${x} ${y}` : `${path} L ${x} ${y}`;
            }, '')"
            fill="none"
            stroke="#396cd8"
            stroke-width="2"
          />
          <circle
            v-for="point in curveData"
            :key="point.day"
            :cx="10 + (point.day - 1) * (700 / (dayList.length - 1 || 1))"
            :cy="140 - point.score * 24"
            r="3"
            fill="#396cd8"
          />
        </svg>
      </div>

      <!-- 3D缩小方格 -->
      <div class="day-grid">
        <div
          v-for="day in dayList"
          :key="day"
          class="day-block"
          :style="{
            background: statusMap[getDayInfo(day).status].color,
            boxShadow: `inset 0 -4px 0 ${statusMap[getDayInfo(day).status].shadow}, 0 2px 8px rgba(0,0,0,0.35)`
          }"
          @click="openEdit(day)"
        >
          <span class="day-num">{{ day }}</span>
          <span class="day-text">{{ getShortText(getDayInfo(day).text) }}</span>
        </div>
      </div>
    </div>

    <!-- 弹窗包含5个状态按钮 -->
    <div class="popup-mask" v-if="showPopup" @click="showPopup = false">
      <div class="popup-box" @click.stop>
        <h3>{{ currentYear }}年{{ currentMonth }}月{{ selectDay }}日 - 填写评价</h3>
        <textarea
          v-model="tempText"
          maxlength="20"
          placeholder="输入当日简短评价（限20字内）"
          rows="4"
        ></textarea>
        <p class="word-count">已输入 {{ tempText.length }} / 20 字</p>
        <div class="btn-group">
          <button
            v-for="(item, idx) in statusMap"
            :key="idx"
            @click="saveRecord(idx as DayStatus)"
            class="status-btn"
            :style="{ borderColor: item.color }"
          >
            {{ item.emoji }} {{ item.label }}
          </button>
        </div>
      </div>
    </div>
  </main>
</template>

<style scoped>
.year-wrap {
  margin-top: 6vh;
  padding: 24px;
  border: 1px solid #ccc;
  border-radius: 12px;
  max-width: 1100px;
  width: 96%;
}

.year-switch {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 12px;
  margin-bottom: 12px;
}

.year-text {
  font-size: 18px;
  font-weight: 500;
}

.leap-tip {
  font-size: 13px;
  opacity: 0.7;
  margin-left: 10px;
}

.month-tabs {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  margin: 16px 0;
  justify-content: center;
}

.month-btn {
  padding: 6px 12px;
  border-radius: 6px;
  border: 1px solid #aaa;
}

.month-btn.active {
  background: #396cd8;
  color: white;
  border-color: #396cd8;
}

.legend {
  display: flex;
  gap: 10px;
  flex-wrap: wrap;
  margin-bottom: 16px;
  justify-content: center;
  font-size: 13px;
}

.color-block {
  display: inline-block;
  width: 14px;
  height: 14px;
  border-radius: 4px;
  margin-right: 4px;
  vertical-align: middle;
}

.chart-box {
  margin: 20px 0;
  padding: 12px;
  border: 1px solid #ddd;
  border-radius: 8px;
}

.chart-box h4 {
  margin: 0 0 8px;
  opacity: 0.8;
  text-align: center;
}

.day-grid {
  display: grid;
  grid-template-columns: repeat(8, 1fr);
  gap: 13px;
  max-width: 760px;
  margin: 0 auto;
  padding: 10px;
}

.day-block {
  width: 100%;
  aspect-ratio: 1 / 1;
  border-radius: 9px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  color: #fff;
  cursor: pointer;
  transition: transform 0.12s ease, box-shadow 0.12s ease;
  text-shadow: 0 0 2px rgba(0, 0, 0, 0.45);
  padding: 2px;
  font-weight: 600;
}

.day-block:hover {
  transform: translateY(-3px) scale(1.03);
}

.day-block:active {
  transform: translateY(-1px) scale(1.01);
  box-shadow: inset 0 -2px 0 rgba(0, 0, 0, 0.4), 0 1px 4px rgba(0, 0, 0, 0.25);
}

.day-num {
  font-size: 13px;
  line-height: 1;
}

.day-text {
  font-size: 9px;
  margin-top: 2px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 90%;
}

.popup-mask {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.5);
  display: grid;
  place-items: center;
  z-index: 99;
}

.popup-box {
  padding: 24px;
  border-radius: 12px;
  background: #fff;
  min-width: 380px;
}

textarea {
  width: 100%;
  box-sizing: border-box;
  margin: 10px 0;
  padding: 10px;
  border-radius: 8px;
  resize: none;
}

.word-count {
  text-align: right;
  font-size: 13px;
  opacity: 0.7;
  margin: -6px 0 12px;
}

.btn-group {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.status-btn {
  border-width: 2px;
}

@media (prefers-color-scheme: dark) {
  .year-wrap {
    border-color: #444;
  }
  .chart-box {
    border-color: #444;
  }
  .popup-box {
    background: #2f2f2f;
  }
}
</style>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;
  color: #0f0f0f;
  background-color: #f6f6f6;
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  padding-left: 12px;
  padding-right: 12px;
}

h2 {
  text-align: center;
}

input,
button,
textarea {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}

button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button,
textarea {
  outline: none;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }
  input,
  button,
  textarea {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}
</style>