<template>
  <div class="time-picker" ref="timePicker">
    <div class="time-input-wrapper">
      <input
        type="text"
        :value="formattedTime"
        @focus="openTimePicker"
        placeholder="Select a time"
        class="time-input"
      />
      <button @click="toggleClock" class="clock-button">
        <span>&#128339;</span> <!-- Clock Icon -->
      </button>
    </div>

    <div v-if="showClock" class="clock-popup">
      <!-- Analog Clock -->
      <div class="analog-clock">
        <div class="clock-face">
          <div
            v-for="hour in 12"
            :key="'hour-' + hour"
            class=hour-mark
            :style="getHourMarkStyle(hour)"
          ></div>
          <div class="clock-hand hour" :style="hourHandStyle"></div>
          <div class="clock-hand minute" :style="minuteHandStyle"></div>
        </div>
      </div>

      <!-- Digital Clock -->
      <div class="digital-clock">
        <div class="time-field">
          <input
            type="number"
            class="time-input-hour"
            v-model="hourInput"
            @blur="syncAnalogClock"
            @input="clampTime"
            :placeholder="'HH'"
          />
          <span class="separator">:</span>
          <input
            type="number"
            class="time-input-minute"
            v-model="minuteInput"
            @blur="syncAnalogClock"
            @input="clampTime"
            :placeholder="'MM'"
          />
        </div>
        <div class="arrows">
          <button @click="incrementHour">▲</button>
          <button @click="decrementHour">▼</button>
          <button @click="incrementMinute">▲</button>
          <button @click="decrementMinute">▼</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
export default {
  name: "TimePicker",
  props: {
    minTime: {
      type: String,
      default: "00:00", // Format HH:mm
    },
    maxTime: {
      type: String,
      default: "23:59", // Format HH:mm
    },
    selectedTime: {
      type: String,
      default: "00:00", // Format HH:mm
    },
  },
  data() {
    return {
      selectedHour: null as number | null,
      selectedMinute: null as number | null,
      showClock: false,
    };
  },
  computed: {
    formattedTime(): string {
      const hour = this.selectedHour?.toString().padStart(2, "0") || this.minTime.split(":")[0];
      const minute = this.selectedMinute?.toString().padStart(2, "0") || this.minTime.split(":")[1];
      return `${hour}:${minute}`;
    },
    hourInput: {
      get(): string {
        return this.selectedHour?.toString() || "";
      },
      set(value: string) {
        this.selectedHour = parseInt(value) || 0;
      },
    },
    minuteInput: {
      get(): string {
        return this.selectedMinute?.toString() || "";
      },
      set(value: string) {
        this.selectedMinute = parseInt(value) || 0;
      },
    },
    hourHandStyle(): string {
      const degrees = (this.selectedHour || 0) * 30 - 180;
      return `transform: rotate(${degrees}deg)`;
    },
    minuteHandStyle(): string {
      const degrees = (this.selectedMinute || 0) * 6 - 180;
      return  `transform: rotate(${degrees}deg)`;
    },
  },
  methods: {
    toggleClock() {
      this.showClock = !this.showClock;
    },
    openTimePicker() {
      this.showClock = true;
    },
    closeClock() {
      this.showClock = false;
    },
    selectHour(hour: number) {
      this.selectedHour = hour;
      this.syncDigitalClock();
    },
    selectMinute(minute: number) {
      this.selectedMinute = minute;
      this.syncDigitalClock();
    },
    clampTime() {
      if (this.selectedMinute < 0) {
        this.selectedMinute = 59;
        this.selectedHour = (this.selectedHour || 0) - 1;
      } else if (this.selectedMinute > 59) {
        this.selectedMinute = 0;
        this.selectedHour = (this.selectedHour || 0) + 1;
      }
      let minTimeHours = parseInt(this.minTime.split(":")[0]);
      let minTimeMinutes = parseInt(this.minTime.split(":")[1]);
      let maxTimeHours = parseInt(this.maxTime.split(":")[0]);
      let maxTimeMinutes = parseInt(this.maxTime.split(":")[1]);
      const minTime = new Date(null, null, null, minTimeHours, minTimeMinutes, 0);
      const maxTime = new Date(null, null, null, maxTimeHours, maxTimeMinutes, 0);
      const selectedTime = new Date(null, null, null, this.selectedHour, this.selectedMinute, 0);

      if (selectedTime < minTime) {
        this.selectedHour = minTime.getHours();
        this.selectedMinute = minTime.getMinutes();
      } else if (selectedTime > maxTime) {
        this.selectedHour = maxTime.getHours();
        this.selectedMinute = maxTime.getMinutes();
      }
    },
    syncAnalogClock() {
      this.clampTime();
    },
    syncDigitalClock() {
      this.clampTime();
    },
    incrementHour() {
      this.selectedHour = (this.selectedHour || 0) + 1;
      this.syncAnalogClock();
    },
    decrementHour() {
      this.selectedHour = (this.selectedHour || 0) - 1;
      this.syncAnalogClock();
    },
    incrementMinute() {
      this.selectedMinute = (this.selectedMinute || 0) + 1;
      this.syncAnalogClock();
    },
    decrementMinute() {
      this.selectedMinute = (this.selectedMinute || 0) - 1;
      this.syncAnalogClock();
    },
    getHourMarkStyle(hour: number): string {
      const degrees = hour * 30; // Each hour is 30 degrees
      return  `transform: rotate(${degrees}deg) translateX(-50%) translateY(-50%)`;
    },
  },
};
</script>

<style scoped>
.time-picker {
  position: relative;
  display: inline-block;
}

.time-input-wrapper {
  display: flex;
  align-items: center;
}

.time-input {
  padding: 5px;
  border: 1px solid var(--accent);
  border-radius: 4px;
  background: var(--bg-dark);
  color: white;
}

.clock-button {
  margin-left: 5px;
  background: none;
  border: none;
  cursor: pointer;
  color: white;
}

.clock-popup {
  position: absolute;
  top: 100%;
  left: 0;
  background: var(--bg-dark);
  border: 1px solid var(--accent);
  padding: 10px;
  border-radius: 5px;
  z-index: 10;
}

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

.clock-hand.minute {
  height: 50%;
  width: 2px;
  background: var(--accent2);
}

.clock-hand.hour {
  height: 35%;
  width: 4px;
  background: var(--accent);
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

.digital-clock {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 10px;
}

.time-field {
  display: flex;
  align-items: center;
  gap: 5px;
}

.time-field input {
  width: 40px;
  padding: 5px;
  font-size: 14px;
  text-align: center;
  border: 1px solid var(--accent);
  border-radius: 4px;
  background: var(--bg-dark);
  color: white;
}

.separator {
  font-size: 18px;
  color: white;
}

.arrows {
  display: flex;
  flex-direction: column;
  gap: 5px;
}

.arrows button {
  width: 25px;
  height: 25px;
  border: none;
  border-radius: 4px;
  background: var(--accent);
  color: white;
  cursor: pointer;
}

.arrows button:hover {
  background: var(--accent-hover);
}
</style>
