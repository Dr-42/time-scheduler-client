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
import { ref, onMounted, onUnmounted } from "vue";

export default {
  props: ["username", "currentStart", "currentName", "currentColor"],
  setup(props) {
    const timer = ref("");

    const updateTimer = () => {
      const now = new Date();
      const start = new Date(props.currentStart);
      timer.value = formatDuration(start, now);
    };

    const formatDuration = (startTime: Date, endTime: Date): string => {
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
    };

    let intervalId: number | undefined;

    onMounted(() => {
      updateTimer();
      intervalId = setInterval(updateTimer, 1000);
    });

    onUnmounted(() => {
      if (intervalId) clearInterval(intervalId);
    });

    return {
      timer,
    };
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
  border: 1px solid #ccc;
  border-radius: 8px;
  background: #2e2e2e;
  margin: 0 auto;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
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
  color: #e2e2e2;
  background-color: v-bind(currentColor);
  padding: 5px 10px;
  border-radius: 5px;
  margin: 10px 0;
}

.bottom {
  font-size: 16px;
  color: #e2e2e2;
}
</style>
