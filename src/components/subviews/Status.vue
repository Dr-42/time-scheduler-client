<template>
  <div class="status">
    <div class="top">
      Welcome {{ username }}
    </div>
    <div class="timer">{{ timer }}</div>
    <div class="bottom">Currently in <b>{{ currentName }}</b></div>
  </div>
</template>

<script lang="ts">
import { Color } from '../../types';

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
  background: var(--bg);
  margin: 0 auto;
  font-family: Arial, sans-serif;
  color: #e2e2e2;
  height: 13vh;
}

.top {
  font-size: 24px;
  font-weight: bold;
  padding: 5px;
}

.timer {
  font-size: 32px;
  font-weight: bold;
  color: #e2e2e2;
  background-color: v-bind("currentCol");
  padding: 10px 10px;
  border-radius: 5px;
}

.bottom {
  font-size: 20px;
  color: #e2e2e2;
}
</style>
