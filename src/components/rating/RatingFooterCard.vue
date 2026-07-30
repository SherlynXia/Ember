<template>
  <div class="bottom-desc-card">
    <h3 class="card-title">✨ 今日金句</h3>
    <p class="quote-text">{{ randomQuote }}</p>
    <div class="decor-dot-group" :style="dotGridStyle">
      <div
        v-for="dotIndex in dotTotalList"
        :key="dotIndex"
        class="decor-dot"
        :style="{ backgroundColor: globalConfig.themePrimaryColor }"
      ></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import type { GlobalThemeConfig } from '../../types/rating'

const quotes = [
  // 励志成长
  '不积跬步，无以至千里。——荀子',
  '千里之行，始于足下。——老子',
  '业精于勤，荒于嬉。——韩愈',
  '天道酬勤。——《尚书》',
  '志当存高远。——诸葛亮',
  '滴水能穿石。——《汉书》',
  '梅花香自苦寒来。——《警世贤文》',
  '世上无难事，只要肯登攀。——毛泽东',
  '少壮不努力，老大徒伤悲。——《乐府诗集》',
  '越努力，越幸运。——励志',
  '做最好的自己。——励志',
  '每天进步一点点。——励志',
  '厚积薄发。——苏轼',
  '不忘初心，方得始终。——《华严经》',
  '知行合一。——王阳明',
  '宁静致远。——诸葛亮',
  // 学习求知
  '学而时习之。——孔子',
  '温故而知新。——孔子',
  '三人行，必有我师。——孔子',
  '知之为知之。——孔子',
  '一寸光阴一寸金。——《增广贤文》',
  '明日复明日，明日何其多。——《明日歌》',
  '书中自有黄金屋。——赵恒',
  '读万卷书，行万里路。——董其昌',
  '活到老，学到老。——刘向',
  '学如逆水行舟，不进则退。——《增广贤文》',
  '书山有路勤为径。——韩愈',
  '学海无涯苦作舟。——韩愈',
  '黑发不知勤学早，白首方悔读书迟。——颜真卿',
  '读书破万卷，下笔如有神。——杜甫',
  // 为人处世
  '己所不欲，勿施于人。——孔子',
  '言必信，行必果。——孔子',
  '人无远虑，必有近忧。——孔子',
  '工欲善其事，必先利其器。——孔子',
  '与朋友交，言而有信。——孔子',
  '满招损，谦受益。——《尚书》',
  '三人行，必有我师焉。——孔子',
  '君子和而不同。——孔子',
  '往者不可谏，来者犹可追。——孔子',
  // 天地自然
  '天行健，自强不息。——《周易》',
  '地势坤，厚德载物。——《周易》',
  '道法自然。——老子',
  '上善若水。——老子',
  '天地不仁，以万物为刍狗。——老子',
  '人法地，地法天，天法道。——老子',
  '大方无隅，大器晚成。——老子',
  '大音希声，大象无形。——老子',
  '祸兮福所倚，福兮祸所伏。——老子',
  '千里之堤，溃于蚁穴。——《韩非子》',
  // 天文宇宙
  '仰望星空，脚踏实地。——励志',
  '宇宙无边，探索无限。——励志',
  '星辰大海，永不止步。——励志',
  '我们都是星尘。——卡尔·萨根',
  '天空没有极限。——励志',
  '于浩瀚宇宙，渺小亦不凡。——励志',
  '心向繁星，追光而行。——励志',
  '地球是摇篮，但人类不会永远待在摇篮里。——齐奥尔科夫斯基',
  '宇宙最不可理解之事，是它竟可被理解。——爱因斯坦',
  '为了那一片星辰。——励志',
  // 诗意生活
  '面朝大海，春暖花开。——海子',
  '生活不止眼前的苟且。——许巍',
  '行到水穷处，坐看云起时。——王维',
  '采菊东篱下，悠然见南山。——陶渊明',
  '长风破浪会有时，直挂云帆济沧海。——李白',
  '天生我材必有用。——李白',
  '会当凌绝顶，一览众山小。——杜甫',
  '山重水复疑无路，柳暗花明又一村。——陆游',
  '不畏浮云遮望眼，只缘身在最高层。——王安石',
  '海内存知己，天涯若比邻。——王勃',
  // 人生哲思
  '人生如逆旅，我亦是行人。——苏轼',
  '此心安处是吾乡。——苏轼',
  '人间有味是清欢。——苏轼',
  '一蓑烟雨任平生。——苏轼',
  '但愿人长久，千里共婵娟。——苏轼',
  '人生自古谁无死，留取丹心照汗青。——文天祥',
  '先天下之忧而忧，后天下之乐而乐。——范仲淹',
  '老骥伏枥，志在千里。——曹操',
  '山不厌高，海不厌深。——曹操',
  '对酒当歌，人生几何。——曹操',
]

const randomQuote = ref('')

onMounted(() => {
  const index = Math.floor(Math.random() * quotes.length)
  randomQuote.value = quotes[index]
})

defineProps<{
  dotGridStyle: Record<string, string>
  dotTotalList: number[]
  globalConfig: GlobalThemeConfig
}>()
</script>

<style scoped>
.bottom-desc-card {
  background-color: v-bind('globalConfig.cardBgColor');
  border-radius: 12px;
  padding: 24px;
  position: relative;
  overflow: hidden;
}
.card-title {
  font-size: 18px;
  margin: 0 0 14px 0;
  font-weight: 500;
  letter-spacing: 2px;
  opacity: 0.75;
}
.quote-text {
  font-size: 22px;
  line-height: 1.7;
  font-weight: 600;
  letter-spacing: 1px;
  font-style: italic;
  margin: 0;
  min-height: 3.4em;
  display: flex;
  align-items: center;
  background: linear-gradient(135deg,
    v-bind('globalConfig.themePrimaryColor'),
    color-mix(in srgb, v-bind('globalConfig.themePrimaryColor') 60%, #fff 40%));
  -webkit-background-clip: text;
  background-clip: text;
  -webkit-text-fill-color: transparent;
}
.decor-dot-group {
  position: absolute;
  right: 60px;
  bottom: 20px;
  width: 240px;
  height: 240px;
  gap: 3px;
  opacity: 0.7;
  pointer-events: none;
  display: grid;
}
.decor-dot {
  border-radius: 2px;
  aspect-ratio: 1/1;
}
@media screen and (max-width: 768px) {
  .decor-dot-group {
    width: 160px;
    height: 160px;
    right: 20px;
    bottom: 10px;
  }
}
</style>