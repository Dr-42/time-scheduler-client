<template>
  <div class="history-time-cards">
    <h2>{{ formattedDate }}</h2>
    <error-display
        v-if="error"
        :errorText="errorText"
    />
    <div id="loading" v-if="loading">
        <loading-spinner />
    </div>
    <time-cards 
      :cardData="cards"
      :blockTypes="blockTypes"
      @split-block="openSplitBlockModal"
      @adjust-block="openAdjustBlockModal"
    />

    <!-- Modals -->
    <split-block-modal
      v-if="currentModal === 'splitBlock'"
      :blockTypes="blockTypes"
      :timeblock="currentActionBlock!"
      @close="currentModal = null"
      @done="handleSplitBlock"
    />
    <adjust-block-modal
      v-if="currentModal === 'adjustBlock'"
      :blockTypes="blockTypes"
      :timeblock="currentActionBlock!"
      :start-time-min="preActionBlockStart"
      :end-time-max="postActionBlockEnd"
      @close="currentModal = null"
      @done="handleAdjustBlock"
    />
  </div>
</template>

<script lang="ts">
import { invoke } from "@tauri-apps/api/core";
import TimeCards from "../components/subviews/TimeCards.vue";

import SplitBlockModal from "../components/modals/SplitBlockModal.vue";
import AdjustBlockModal from "../components/modals/AdjustBlockModal.vue";

import LoadingSpinner from "../components/subviews/LoadingSpinner.vue";
import ErrorDisplay from "../components/subviews/ErrorDisplay.vue";

import { BlockType, HistoryData, TimeBlock } from "../types";

type SplitBlockModalData = {
  splitTime: string;
  beforeTitle: string;
  beforeBlockType: number;
  afterTitle: string;
  afterBlockType: number;
};

type AdjustBlockModalData = {
  title: string;
  blockType: number;
  newStartTime: string;
  newEndTime: string;
};

export default {
  name: "HistoryTimeCards",
  components: { 
    TimeCards,

    SplitBlockModal,
    AdjustBlockModal,

    LoadingSpinner,
    ErrorDisplay,
  },
  data() {
    return {
      loading: false,
      error: false,
      errorText: {},
      currentModal: null as string | null,
      cards: [] as TimeBlock[],
      blockTypes: [] as BlockType[],
      fetched: false,
      currentActionBlock: null as TimeBlock | null,
      preActionBlock: null as TimeBlock | null,
      postActionBlock: null as TimeBlock | null,
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
    preActionBlockStart() {
      if (this.preActionBlock) {
        const start = new Date(this.preActionBlock.startTime);
        const startHour = start.getHours();
        const startMinute = start.getMinutes();
        const startSecond = start.getSeconds();
        const timeString = startHour.toString().padStart(2, "0") + ":" + startMinute.toString().padStart(2, "0") + ":" + startSecond.toString().padStart(2, "0");
        return timeString;
      } else {
        return "00:00:00";
      }
    },
    postActionBlockEnd() {
      if (this.postActionBlock) {
        const end = new Date(this.postActionBlock.endTime);
        const endHour = end.getHours();
        const endMinute = end.getMinutes();
        const endSecond = end.getSeconds();
        const timeString = endHour.toString().padStart(2, "0") + ":" + endMinute.toString().padStart(2, "0") + ":" + endSecond.toString().padStart(2, "0");
        return timeString;
      } else {
        const now = new Date();
        console.log(now);
        const endHour = now.getHours();
        const endMinute = now.getMinutes();
        const endSecond = now.getSeconds();
        const timeString = endHour.toString().padStart(2, "0") + ":" + endMinute.toString().padStart(2, "0") + ":" + endSecond.toString().padStart(2, "0");
        return timeString;
      }
    },
  },
  methods: {
    openSplitBlockModal(block: any) {
      this.currentActionBlock = TimeBlock.fromObject(block);
      this.currentModal = "splitBlock";
    },
    async handleSplitBlock(data: SplitBlockModalData) {
      try {
        this.loading = true;
        const sendData = {
          start_time: this.currentActionBlock!.startTime,
          end_time: this.currentActionBlock!.endTime,
          split_time: data.splitTime,
          before_title: data.beforeTitle,
          before_block_type_id: data.beforeBlockType,
          after_title: data.afterTitle,
          after_block_type_id: data.afterBlockType,
        };
        await invoke("post_split_block", { data : sendData });
        this.currentActionBlock = null;
        this.currentModal = null;
        this.loading = false;
        await this.reloadPage();
      } catch (e) {
        console.error(e);
        this.error = true;
        this.errorText = e as string;
      }
    },
    openAdjustBlockModal(blocks: any) {
      this.currentActionBlock = TimeBlock.fromObject(blocks.card);
      if (blocks.pre) {
        this.preActionBlock = TimeBlock.fromObject(blocks.pre);
      } else {
        this.preActionBlock = null;
      }
      if (blocks.post) {
        this.postActionBlock = TimeBlock.fromObject(blocks.post);
      } else {
        this.postActionBlock = null;
      }
      this.currentModal = "adjustBlock";
    },
    async handleAdjustBlock(data: AdjustBlockModalData) {
      try {
        this.loading = true;
        const sendData = {
          start_time: this.currentActionBlock!.startTime,
          end_time: this.currentActionBlock!.endTime,
          new_start_time: data.newStartTime,
          new_end_time: data.newEndTime,
          title: data.title,
          block_type_id: data.blockType,
        };
        await invoke("post_adjust_block", { data : sendData });
        this.currentActionBlock = null;
        this.currentModal = null;
        this.loading = false;
        await this.reloadPage();
      } catch (e) {
        console.error(e);
        this.error = true;
        this.errorText = e as string;
      }
    },

    async reloadPage() {
      try{
        const dateStr = this.$route.params.date as string;
        let date = new Date(dateStr);
        let historyData = await invoke("get_day_history", { date: date });
        let history = HistoryData.fromJson(historyData);
        this.cards = history.daydata;
        this.blockTypes = history.blocktypes;
        this.fetched = true;
      } catch (e) {
        console.error(e);
        this.error = true;
        this.errorText = e as string;
      }
    }
  },

  async beforeCreate() {
    const dateStr = this.$route.params.date as string;
    let date = new Date(dateStr);
    let historyData = await invoke("get_day_history", { date: date });
    let history = HistoryData.fromJson(historyData);
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
  /* Center */
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
}

h2 {
  text-align: center;
  margin: 0;
  padding: 0;
  z-index: 8;
  position: sticky;
  background-color: #2e2e2e;
  border-bottom: 1px solid #e2e2e2;
  top: 0;
}
</style>
