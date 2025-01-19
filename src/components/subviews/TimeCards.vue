<template>
  <div class="time-cards">
    <div 
      v-for="(card, index) in cardData" 
      :key="index"
      class="card-wrapper"
      @contextmenu.prevent="openContextMenu($event, card)"
    >
      <time-card 
        :blockname="card.title" 
        :startTime="card.startTime" 
        :endTime="card.endTime" 
        :color="getCardColor(card.blockTypeId, blockTypes)"
      />
    </div>
    <div 
      v-if="showContextMenu" 
      class="context-menu" 
      :style="{ top: `${contextMenuPosition.y}px`, left: `${contextMenuPosition.x}px` }"
    >
      <ul>
        <li @click="splitBlock(selectedCard)">Split block</li>
        <li @click="adjustBlock(selectedCard, preCard, postCard)">Adjust block</li>
      </ul>
    </div>
  </div>
</template>

<script lang="ts">
import { BlockType, Color, TimeBlock } from '../../types';
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
  emits: ["splitBlock", "adjustBlock"],
  data() {
    return {
      showContextMenu: false,
      contextMenuPosition: { x: 0, y: 0 },
      selectedCard: null as TimeBlock | null,
      preCard: null as TimeBlock | null,
      postCard: null as TimeBlock | null
    };
  },
  methods: {
    getCardColor(blockId: number, blockTypes: Array<BlockType>) : Color {
      const blockType = blockTypes.find((blockType) => blockType.id === blockId);
      if (blockType) {
        return blockType.color;
      } else {
        return new Color(0, 0, 0);
      }
    },
    openContextMenu(event: MouseEvent, card: TimeBlock) {
      this.showContextMenu = true;
      this.contextMenuPosition = { x: event.clientX, y: event.clientY };
      this.selectedCard = card;
      let preCard = this.cardData[this.cardData.indexOf(card) + 1];
      let postCard = this.cardData[this.cardData.indexOf(card) - 1];
      this.preCard = preCard;
      this.postCard = postCard;
    },
    closeContextMenu() {
      this.showContextMenu = false;
      this.selectedCard = null;
    },
    splitBlock(card: any) {
      this.$emit("splitBlock", card);
      this.closeContextMenu();
    },
    adjustBlock(card: any, preCard: any, postCard: any) {
      console.log("Adjusting block:", card);
      this.$emit("adjustBlock", {
        card: card,
        pre: preCard,
        post: postCard
      });
      this.closeContextMenu();
    }
  },
  mounted() {
    // Close context menu on clicking outside
    window.addEventListener('click', this.closeContextMenu);
  },
  beforeDestroy() {
    window.removeEventListener('click', this.closeContextMenu);
  }
};
</script>

<style scoped>
.time-cards {
  display: flex;
  flex-direction: column;
  justify-content: start;
  padding: 2px;
  background: var(--bg);
  margin: 0;
  width: 100%;
  font-family: Arial, sans-serif;
  color: #e2e2e2;
  height: 100%;
  overflow-y: auto;
}

@media (orientation: portrait) {
  .time-cards {
    border-top: 1px solid #ccc;
  }
}

@media (orientation: landscape) {
  .time-cards {
    border-left: 1px solid #ccc;
  }
}

.card-wrapper {
  width: 100%;
}

.context-menu {
  position: absolute;
  background: #333;
  color: white;
  border: 1px solid #666;
  border-radius: 4px;
  z-index: 1000;
  box-shadow: 0px 4px 6px rgba(0, 0, 0, 0.3);
}

.context-menu ul {
  list-style: none;
  margin: 0;
  padding: 5px 0;
}

.context-menu li {
  padding: 8px 12px;
  cursor: pointer;
}

.context-menu li:hover {
  background: #555;
}
</style>
