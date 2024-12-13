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
export default {
  props: ["username", "currentStart", "currentName", "currentColor"],
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
    },
    colorToString(color: { r: number; g: number; b: number }) {
      return `rgb(${color.r}, ${color.g}, ${color.b})`;
    },
    contrastColor(color: { r: number; g: number; b: number }) {
      const r = color.r;
      const g = color.g;
      const b = color.b;
      const yiq = ((r * 299) + (g * 587) + (b * 114)) / 1000;
      return yiq >= 128 ? "#000000" : "#ffffff";
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
  color: v-bind(contrastColor(currentColor));
  background-color: v-bind(colorToString(currentColor));
  padding: 5px 10px;
  border-radius: 5px;
  margin: 10px 0;
}

.bottom {
  font-size: 16px;
  color: #e2e2e2;
}
</style>
