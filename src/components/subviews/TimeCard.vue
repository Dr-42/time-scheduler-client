<template>
  <div class="time-card">
    <div class="left">
      <div class="name">{{ blockname }}</div>
      <div class="time">{{ fmtStartTime }} - {{ fmtEndTime }}</div>
    </div>
    <div class="right">
      <div class="duration">{{ duration }}</div>
    </div>
  </div>
</template>

<script lang="ts">
import { Color } from '../../types';

export default {
  props: {
    blockname: {
      type: String,
      required: true,
    },
    startTime: {
      type: String,
      required: true,
    },
    endTime: {
      type: String,
      required: true,
    },
    color: {
      type: Color,
      required: true,
    },
  },
  computed: {
    fmtStartTime() {
      return this.formatTime(this.startTime);
    },
    fmtEndTime() {
      return this.formatTime(this.endTime);
    },
    duration() {
      return this.formatDuration(this.startTime, this.endTime);
    },
    col() {
      return this.color.toString();
    }
  },
  methods: {
    formatTime(timeValue: string) {
      let time = new Date(timeValue);
      const hours = time.getHours().toString().padStart(2, "0");
      const minutes = time.getMinutes().toString().padStart(2, "0");
      return `${hours}:${minutes}`;
    },
    formatDuration(startTimeValue: string, endTimeValue: string) {
      let startTime = new Date(startTimeValue);
      let endTime = new Date(endTimeValue);
      const duration = endTime.getTime() - startTime.getTime();
      const hours = Math.floor(duration / (1000 * 60 * 60));
      const minutes = Math.floor((duration % (1000 * 60 * 60)) / (1000 * 60));
      const seconds = Math.floor((duration % (1000 * 60 * 60)) % (1000 * 60) / 1000);
      if (hours === 0 && minutes === 0) {
        return `${seconds}s`;
      } else if (hours === 0) {
        return `${minutes}m ${seconds}s`;
      } else {
        return `${hours}h ${minutes}m ${seconds}s`;
      }
    }
  }
}
</script>

<style scoped>
.time-card {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  margin: 2.5px 0 2.5px 0;
  width: 100%;
  align-items: center;
  background-color: v-bind(col);
  border-radius: 5px;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.left {
  flex: 1;
  flex-direction: column;
  padding: 10px;
  text-align: left;
  color: #fff;
}

.name {
  font-weight: bold;
  font-size: 18px;
}

.time {
  font-size: 14px;
}

.right {
  flex: 1;
  padding: 10px;
  text-align: right;
  color: #fff;
  font-weight: bold;
  font-size: 20px;
}
</style>
