<template>
  <div class="time-picker" ref="timePicker">
    <div class="time-input-wrapper">
      <div class="time-input">
        <input
          type="text"
          :value="formattedTime"
          @focus="openTimePicker"
        />
        <button type="button" @click="toggleClock" class="clock-button">
          <clock-edit-icon />
        </button>
      </div>
    </div>

    <div 
      v-if="showClock" 
      class="clock-popup"
      :style="{
        maxHeight: maxDropdownHeight + 'px',
        overflowY: 'auto',
        transform: openAbove ? 'translateY(-100%)' : 'translateY(0)',
      }"
    >
      <!-- Analog Clock -->
      <div class="analog-clock">
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
      </div>

      <!-- Digital Clock -->
      <div class="digital-clock">
        <div class="time-field-wrapper">
          <div class="time-field">
            <div class="hour-field">
              <input
                type="number"
                class="time-input-hour"
                v-model="hourInput"
                @blur="syncAnalogClock"
                @input="clampTime"
                :placeholder="minTime.split(':')[0]"
              />
              <div class="arrows">
                <button type="button" @click="incrementHour">▲</button>
                <button type="button" @click="decrementHour">▼</button>
              </div>
            </div>
            <span class="separator">:</span>
            <div class="minute-field">
              <input
                type="number"
                class="time-input-minute"
                v-model="minuteInput"
                @blur="syncAnalogClock"
                @input="clampTime"
                :placeholder="minTime.split(':')[1]"
              />
              <div class="arrows">
                <button type="button" @click="incrementMinute">▲</button>
                <button type="button" @click="decrementMinute">▼</button>
              </div>
            </div>
            <div class="second-field">
              <input
                type="number"
                class="time-input-second"
                v-model="secondInput"
                @blur="syncAnalogClock"
                @input="clampTime"
                :placeholder="minTime.split(':')[1]"
              />
              <div class="arrows">
                <button type="button" @click="incrementSecond">▲</button>
                <button type="button" @click="decrementSecond">▼</button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import ClockEditIcon from "vue-material-design-icons/ClockEdit.vue"

export default {
  name: "TimePicker",
  components: { ClockEditIcon },
  props: {
    minTime: {
      type: String,
      default: "00:00:00",
    },
    maxTime: {
      type: String,
      default: "23:59:00",
    },
    selectedTime: {
      type: String,
      default: "00:00:00",
    },
  },
  emits: ["update:selectedTime"],
  data() {
    return {
      selectedHour: 0,
      selectedMinute: 0,
      selectedSecond: 0,
      showClock: false,
      openAbove: false,
      maxDropdownHeight: 300,
    };
  },
  computed: {
    formattedTime(): string {
      const hour = this.selectedHour.toString().padStart(2, "0") || this.minTime.split(":")[0];
      const minute = this.selectedMinute.toString().padStart(2, "0") || this.minTime.split(":")[1];
      const second = this.selectedSecond.toString().padStart(2, "0") || this.minTime.split(":")[2];
      return `${hour}:${minute}:${second}`;
    },
    hourInput: {
      get(): string {
        if (this.selectedHour < 10) {
          return `0${this.selectedHour}`;
        } else {
          return this.selectedHour.toString();
        }
      },
      set(value: string) {
        this.selectedHour = parseInt(value) || 0;
      },
    },
    minuteInput: {
      get(): string {
        if (this.selectedMinute < 10) {
          return `0${this.selectedMinute}`;
        } else {
          return this.selectedMinute.toString();
        }
      },
      set(value: string) {
        this.selectedMinute = parseInt(value) || 0;
      },
    },
    secondInput: {
      get(): string {
        if (this.selectedSecond < 10) {
          return `0${this.selectedSecond}`;
        } else {
          return this.selectedSecond.toString();
        }
      },
      set(value: string) {
        this.selectedSecond = parseInt(value) || 0;
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
    secondHandStyle(): string {
      const degrees = (this.selectedSecond || 0) * 6 - 180;
      return `transform: rotate(${degrees}deg)`;
    },
  },
  methods: {
    toggleClock() {
      this.calculateClockPosition();
      this.showClock = !this.showClock;
    },
    openTimePicker() {
      this.calculateClockPosition();
      this.showClock = true;
    },
    closeClock() {
      this.calculateClockPosition();
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
    selectSecond(second: number) {
      this.selectedSecond = second;
      this.syncDigitalClock();
    },
    clampTime() {
      if (this.selectedSecond < 0) {
        this.selectedSecond = 59;
        this.selectedMinute = (this.selectedMinute || 0) - 1;
      } else if (this.selectedSecond > 59) {
        this.selectedSecond = 0;
        this.selectedMinute = (this.selectedMinute || 0) + 1;
      }

      if (this.selectedMinute < 0) {
        this.selectedMinute = 59;
        this.selectedHour = (this.selectedHour || 0) - 1;
      } else if (this.selectedMinute > 59) {
        this.selectedMinute = 0;
        this.selectedHour = (this.selectedHour || 0) + 1;
      }
      let minTimeHours = parseInt(this.minTime.split(":")[0]);
      let minTimeMinutes = parseInt(this.minTime.split(":")[1]);
      let minTimeSeconds = parseInt(this.minTime.split(":")[2]);
      let maxTimeHours = parseInt(this.maxTime.split(":")[0]);
      let maxTimeMinutes = parseInt(this.maxTime.split(":")[1]);
      let maxTimeSeconds = parseInt(this.maxTime.split(":")[2]);
      const minTime = new Date(0, 0, 0, minTimeHours, minTimeMinutes, minTimeSeconds);
      const maxTime = new Date(0, 0, 0, maxTimeHours, maxTimeMinutes, maxTimeSeconds);
      const selectedTime = new Date(0, 0, 0, this.selectedHour, this.selectedMinute, this.selectedSecond);

      if (selectedTime < minTime) {
        this.selectedHour = minTime.getHours();
        this.selectedMinute = minTime.getMinutes();
        this.selectedSecond = minTime.getSeconds();
      } else if (selectedTime > maxTime) {
        this.selectedHour = maxTime.getHours();
        this.selectedMinute = maxTime.getMinutes();
        this.selectedSecond = maxTime.getSeconds();
      }
      this.$emit("update:selectedTime", `${this.selectedHour}:${this.selectedMinute}:${this.selectedSecond}`);
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
    incrementSecond() {
      this.selectedSecond = (this.selectedSecond || 0) + 1;
      this.syncAnalogClock();
    },
    decrementSecond() {
      this.selectedSecond = (this.selectedSecond || 0) - 1;
      this.syncAnalogClock();
    },
    getHourMarkStyle(hour: number): string {
      const degrees = hour * 30; // Each hour is 30 degrees
      return  `transform: rotate(${degrees}deg) translateX(-50%) translateY(-50%)`;
    },
    getMinuteMarkStyle(minute: number): string {
      const degrees = minute * 6; // Each minute is 6 degrees
      return  `transform: rotate(${degrees}deg) translateX(-50%) translateY(-50%)`;
    },
    handleClickOutside(event: MouseEvent) {
      if (
        this.$refs.timePicker &&
        !(this.$refs.timePicker as HTMLElement).contains(event.target as Node)
      ) {
        this.closeClock();
      }
    },
    calculateClockPosition() {
      const clockPopup = this.$refs.timePicker as HTMLElement;
      if (clockPopup) {
        const rect = clockPopup.getBoundingClientRect();
        const spaceBelow = window.innerHeight - rect.bottom;
        const spaceAbove = rect.top;

        // Check if there's enough space below, otherwise open above
        this.openAbove = spaceBelow < this.maxDropdownHeight && spaceAbove > spaceBelow;
      }
    },
  },
  mounted() {
    this.selectedHour = parseInt(this.selectedTime.split(":")[0]);
    this.selectedMinute = parseInt(this.selectedTime.split(":")[1]);
    this.selectedSecond = parseInt(this.selectedTime.split(":")[2]);
    this.syncAnalogClock();
    document.addEventListener("click", this.handleClickOutside);
  },
  beforeDestroy() {
    document.removeEventListener("click", this.handleClickOutside);
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
  padding: 5px;
}

.time-input {
  display: flex;
  justify-content: space-between;
  flex-direction: row;
  align-items: flex-end;
  padding: 5px;
  border: 1px solid var(--accent);
  border-radius: 4px;
  background: var(--bg-dark);
  color: white;
  font-size: 20px;
  text-align: right;
  width: 100%;
}

.time-input input {
  flex-grow: 1;
  padding: 5px;
  border: none;
  background: var(--bg-dark);
  color: white;
  font-size: 20px;
  text-align: right;
  appearance: none;
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
  right: 0;
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

.digital-clock {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-top: 10px;
  color: white;
  background-color: var(--bg);
}

.time-field {
  display: flex;
  align-items: center;
  gap: 10px; /* Increase spacing between inputs */
}

.time-field input {
  padding: 5px;
  font-size: 14px;
  text-align: center;
  border: 1px solid var(--accent);
  border-radius: 4px;
  background: var(--bg-dark);
  color: white;
  flex-grow: 1;
  -webkit-appearance: none;
  -moz-appearance: textfield;
  appearance: textfield;
}

.hour-field,
.minute-field,
.second-field {
  display: flex;
  align-items: center;
  flex-direction: row;
}

.time-field input::-webkit-inner-spin-button,
.time-field input::-webkit-outer-spin-button {
  -webkit-appearance: none;
  margin: 0;
}

.arrows {
  display: flex;
  flex-direction: column; /* Arrange the buttons horizontally */
  gap: 0; /* Remove gap between the arrows */
  margin-left: 5px; /* Add a small margin to separate from the input */
  flex-grow: 0;
  height: 100%;
}

.arrows button {
  border: none;
  border-left: 1px solid var(--accent); /* Add a separator to make it visually connected */
  background: var(--accent);
  color: white;
  font-size: 7px;
  cursor: pointer;
}

.arrows button:first-child {
  border-radius: 0 4px 0 0; /* Top right radius */
}

.arrows button:last-child {
  border-radius: 0 0 4px 0; /* Bottom right radius */
}

.arrows button:hover {
  background: var(--accent-hover);
}

.time-field {
  display: flex;
  align-items: center;
  gap: 10px;
}

.time-field input {
  width: 50px;
  padding: 5px;
  font-size: 14px;
  text-align: center;
  border: 1px solid var(--accent);
  border-radius: 4px 0 0 4px; /* Round only the left side */
  background: var(--bg-dark);
  color: white;
  -webkit-appearance: none;
  -moz-appearance: textfield;
  appearance: textfield;
}
</style>
