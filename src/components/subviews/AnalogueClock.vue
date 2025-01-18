<template>
  <div class="clock-face">
    <div
      v-for="hour in 12"
      :key="'hour-' + hour"
      class=hour-mark
      :style="getHourMarkStyle(hour)"
    ></div>
    <div
      v-for="minute in 60"
      :key="'hour-' + minute"
      class=minute-mark
      :style="getMinuteMarkStyle(minute)"
    ></div>
    <div class="clock-hand hour" :style="hourHandStyle"></div>
    <div class="clock-hand minute" :style="minuteHandStyle"></div>
    <div class="clock-hand second" :style="secondHandStyle"></div>
  </div>
</template>

<script lang="ts">
export default {
  props: {
    time: {
      type: String,
      required: true
    }
  },
  computed: {
    hour(): number {
      return parseInt(this.time.split(':')[0]);
    },
    minute(): number {
      return parseInt(this.time.split(':')[1]);
    },
    second(): number {
      return parseInt(this.time.split(':')[2]);
    },
    hourHandStyle(): string {
      const degrees = (this.hour || 0) * 30 - 180;
      return `transform: rotate(${degrees}deg)`;
    },
    minuteHandStyle(): string {
      const degrees = (this.minute || 0) * 6 - 180;
      return  `transform: rotate(${degrees}deg)`;
    },
    secondHandStyle(): string {
      const degrees = (this.second || 0) * 6 - 180;
      return `transform: rotate(${degrees}deg)`;
    },
  },
  methods: {
    getHourMarkStyle(hour: number): string {
      const degrees = hour * 30; // Each hour is 30 degrees
      return  `transform: rotate(${degrees}deg) translateX(-50%) translateY(-50%)`;
    },
    getMinuteMarkStyle(minute: number): string {
      const degrees = minute * 6; // Each minute is 6 degrees
      return  `transform: rotate(${degrees}deg) translateX(-50%) translateY(-50%)`;
    },
  }
}
</script>

<style scoped>
.analog-clock .clock-face {
  position: relative;
  width: 150px;
  height: 150px;
  border-radius: 50%;
  background: var(--bg);
  border: 2px solid var(--accent);
  margin: 0 auto 10px;
}

.clock-hand {
  position: absolute;
  top: 50%;
  left: 50%;
  transform-origin: center top;
}

.clock-hand.hour {
  height: 35%;
  width: 5px;
  background: var(--accent);
}

.clock-hand.minute {
  height: 50%;
  width: 3px;
  background: var(--accent2);
}

.clock-hand.second {
  height: 50%;
  width: 1px;
  background: var(--accent-hover);
}

.hour-mark {
  position: absolute;
  top: 50%;
  left: 50%;
  width: 2px;
  height: 100%;
  background: linear-gradient(transparent 90%, var(--accent2));
  transform-origin: center top;
}

.minute-mark {
  position: absolute;
  top: 50%;
  left: 50%;
  width: 1px;
  height: 100%;
  background: linear-gradient(transparent 95%, var(--accent-hover));
  transform-origin: center top;
}
</style>
