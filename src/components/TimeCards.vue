<template>
  <div class="time-cards">
    <div v-for="card in cardData">
      <TimeCard 
        :blockname="card.blockname" 
        :startTime="card.startTime" 
        :endTime="card.endTime" 
        :color="card.color" 
      />
    </div>
  </div>
</template>

<script lang="ts">
import TimeCard from './TimeCard.vue';

type Card = {
  blockname: string;
  startTime: string;
  endTime: string;
  color: string;
}

export default {
  name: "TimeCards",
  components: {
    TimeCard
  },
  props: 
  {
    cardData: {
      type: Array<Card>,
      required: true
    }
  }
}
</script>

<style scoped>
.time-cards {
  display: flex;
  flex-direction: column;
  justify-content: center;
  padding: 2px;
  border-top: 1px solid #ccc;
  background: #2e2e2e;
  margin: 0 auto;
  font-family: Arial, sans-serif;
  color: #e2e2e2;
  overflow-y: auto;
}
</style>
