<template>
  <div class="history-time-cards">
    <h2>Timecards for {{ formattedDate }}</h2>
    <time-cards 
      :cardData="cards"
      :blockTypes="blockTypes"
    />
  </div>
</template>

<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import TimeCards from "../components/TimeCards.vue";
import { BlockType, HistoryData, TimeBlock } from "../types";

export default {
  name: "HistoryTimeCards",
  components: { TimeCards },
  data() {
    return {
      cards: [] as TimeBlock[],
      blockTypes: [] as BlockType[],
      fetched: false,
    };
  },
  computed: {
    formattedDate() {
      const dateStr = this.$route.params.date as string;
      const date = new Date(dateStr);
      return date.toLocaleDateString(undefined, {
        year: "numeric",
        month: "long",
        day: "numeric",
      });
    },
  },

  async beforeCreate() {
    const dateStr = this.$route.params.date as string;
    let date = new Date(dateStr);
    let historyData = await invoke("get_day_history", { date: date });
    let history = HistoryData.fromJson(historyData);
    console.log(history);
    this.cards = history.daydata;
    this.blockTypes = history.blocktypes;
    this.fetched = true;
  },
};
</script>

<style scoped>
.history-time-cards {
  padding: 20px;
  color: #e2e2e2;
  font-family: Arial, sans-serif;
}

h2 {
  text-align: center;
  margin-bottom: 20px;
}
</style>
