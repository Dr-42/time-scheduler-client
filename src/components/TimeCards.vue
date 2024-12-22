<template>
  <div class="time-cards">
    <div v-for="card in cardData">
      <TimeCard 
        :blockname="card.title" 
        :startTime="card.startTime" 
        :endTime="card.endTime" 
        :color="getCardColor(card.blockTypeId, blockTypes)" 
      />
    </div>
  </div>
</template>

<script lang="ts">
import TimeCard from './TimeCard.vue';

export default {
  name: "TimeCards",
  components: {
    TimeCard
  },
  props: 
  {
    cardData: {
      type: Array<TimeBlock>,
      required: true
    },
    blockTypes: {
      type: Array<BlockType>,
      required: true
    }
  },
  methods: {
    getCardColor(blockId: number, blockTypes: Array<BlockType>) {
      const blockType = blockTypes.find((blockType) => blockType.id === blockId);
      return blockType ? `rgb(${blockType.color.r}, ${blockType.color.g}, ${blockType.color.b})` : "rgb(0, 0, 0)";
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
