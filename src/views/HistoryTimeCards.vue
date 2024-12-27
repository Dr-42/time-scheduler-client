<template>
  <div class="history-time-cards">
    <h2>Timecards for {{ formattedDate }}</h2>
    <time-cards :cardData="filteredCards" />
  </div>
</template>

<script lang="ts">
import TimeCards from "../components/TimeCards.vue";

type Card = {
  blockname: string;
  startTime: string;
  endTime: string;
  color: string;
};

export default {
  name: "HistoryTimeCards",
  components: { TimeCards },
  data() {
    return {
      allCards: [
        { blockname: "Block 1", startTime: "2024-12-14T09:00:00", endTime: "2024-12-14T10:00:00", color: "#3e3e3e" },
        { blockname: "Block 1", startTime: "2024-12-14T19:00:00", endTime: "2024-12-14T20:00:00", color: "#e23e3e" },
        { blockname: "Block 2", startTime: "2024-12-15T11:00:00", endTime: "2024-12-15T12:30:00", color: "#575757" },
      ],
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
    filteredCards() {
      const selectedDate = this.$route.params.date;
      return this.allCards.filter((card: Card) => {
        const cardDate = card.startTime.split("T")[0];
        return cardDate === selectedDate;
      });
    },
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
