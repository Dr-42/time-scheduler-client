<template>
  <div class="status">
    <div class="top">
      Hello {{ username }}
    </div>
    <div class="timer">{{ timer }}</div>
    <div class="bottom">Currently in {{ currentName }}</div>
  </div>
</template>

<script lang="ts">
import { Color } from '../types';

export default {
  props: {
    username: {
      type: String,
      required: true,
    },
    currentStart: {
        type: String,
        default: "No start time available",
    },
    currentName: {
      type: String,
      required: true,
    },
    currentColor: {
      type: Color,
      required: true,
    },
  },
  computed: {
    currentCol() : string {
      return this.currentColor.toString();
    },
    contrastCol() : string {
      return this.currentColor.contrastColor();
    },
  },
  data() {
    return {
      timer: "",
      intervalId: null as number | null,
    };
  },
  methods: {
    updateTimer() {
      const now = new Date();
      const start = new Date(this.currentStart);
      this.timer = this.formatDuration(start, now);
    },
    formatDuration(startTime: Date, endTime: Date): string {
      const duration = endTime.getTime() - startTime.getTime();
      const hours = Math.floor(duration / (1000 * 60 * 60));
      const minutes = Math.floor((duration % (1000 * 60 * 60)) / (1000 * 60));
      const seconds = Math.floor((duration % (1000 * 60)) / 1000);

      if (hours === 0 && minutes === 0) {
        return `${seconds}s`;
      } else if (hours === 0) {
        return `${minutes}m ${seconds}s`;
      } else {
        return `${hours}h ${minutes}m ${seconds}s`;
      }
    }
  },
  mounted() {
    this.updateTimer();
    this.intervalId = setInterval(this.updateTimer, 1000) as unknown as number;
  },
  beforeUnmount() {
    if (this.intervalId) clearInterval(this.intervalId);
  },
};
</script>

<style scoped>
.status {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 20px;
  border-bottom: 1px solid #ccc;
  background: #2e2e2e;
  margin: 0 auto;
  font-family: Arial, sans-serif;
  color: #e2e2e2;
}

.top {
  font-size: 18px;
  font-weight: bold;
  margin-bottom: 10px;
}

.timer {
  font-size: 24px;
  font-weight: bold;
  color: v-bind("contrastCol");
  background-color: v-bind("currentCol");
  padding: 5px 10px;
  border-radius: 5px;
  margin: 10px 0;
}

.bottom {
  font-size: 16px;
  color: #e2e2e2;
}
</style>
