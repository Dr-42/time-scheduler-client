<template>
  <div class="date-picker" ref="datePicker">
    <input
      type="text"
      :value="formattedDate"
      @input="onInput($event)"
      placeholder="Select a date"
      class="date-input"
    />
    <button @click="toggleCalendar" class="calendar-button">
      <span>&#128197;</span> <!-- Calendar Icon -->
    </button>

    <div v-if="showCalendar" class="calendar-popup">
      <div class="calendar-header">
        <button @click="prevMonth">&lt;</button>
        <span>{{ monthNames[currentMonth] }} {{ currentYear }}</span>
        <button @click="nextMonth">&gt;</button>
      </div>

      <div class="calendar-grid">
        <div class="day-header" v-for="day in dayNames" :key="day">{{ day }}</div>

        <div
          v-for="(date, index) in daysInMonth"
          :key="index"
          :class="['day', { 'disabled': isDisabled(date), 'selected': isSelected(date) }]"
          @click="selectDate(date)"
        >
          {{ date.getDate() }}
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">

export default {
  name: "DatePicker",
  emits: ["date-selected"],
  props: {
    format: {
      type: String,
      default: "DD-MM-YYYY",
    },
    minDate: {
      type: Date,
      default: null,
    },
    maxDate: {
      type: Date,
      default: null,
    },
  },
  data() {
    const today = new Date();
    return {
      selectedDate: null as Date | null,
      showCalendar: false,
      currentMonth: today.getMonth(),
      currentYear: today.getFullYear(),
      monthNames: [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
      ],
      dayNames: ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"],
    };
  },
  computed: {
    formattedDate(): string {
      if (!this.selectedDate) return "";
      return this.formatDate(this.selectedDate, this.format);
    },
    daysInMonth(): Date[] {
      const days: Date[] = [];
      const firstDay = new Date(this.currentYear, this.currentMonth, 1);
      const lastDay = new Date(this.currentYear, this.currentMonth + 1, 0);
      for (let d = new Date(firstDay); d <= lastDay; d.setDate(d.getDate() + 1)) {
        days.push(new Date(d));
      }
      return days;
    },
  },
  methods: {
    formatDate(date: Date, format: string): string {
      const day = date.getDate().toString().padStart(2, "0");
      const month = (date.getMonth() + 1).toString().padStart(2, "0");
      const year = date.getFullYear();

      return format
        .replace("DD", day)
        .replace("MM", month)
        .replace("YYYY", year.toString())
        .replace("YY", year.toString().slice(-2));
    },
    parseDate(dateString: string): Date | null {
      const parts = dateString.split(/[-/]/);
      if (this.format === "DD-MM-YYYY") {
        return new Date(+parts[2], +parts[1] - 1, +parts[0]);
      } else if (this.format === "MM-DD-YYYY") {
        return new Date(+parts[2], +parts[0] - 1, +parts[1]);
      } else if (this.format === "YYYY-MM-DD") {
        return new Date(+parts[0], +parts[1] - 1, +parts[2]);
      }
      return null;
    },
    isDisabled(date: Date): boolean {
      if (this.minDate && date < this.minDate) return true;
      if (this.maxDate && date > this.maxDate) return true;
      return false;
    },
    isSelected(date: Date): boolean {
      return (
        !!this.selectedDate &&
        date.toDateString() === this.selectedDate.toDateString()
      );
    },
    toggleCalendar(): void {
      this.showCalendar = !this.showCalendar;
    },
    closeCalendar(): void {
      this.showCalendar = false;
    },
    prevMonth(): void {
      if (this.currentMonth === 0) {
        this.currentMonth = 11;
        this.currentYear -= 1;
      } else {
        this.currentMonth -= 1;
      }
    },
    nextMonth(): void {
      if (this.currentMonth === 11) {
        this.currentMonth = 0;
        this.currentYear += 1;
      } else {
        this.currentMonth += 1;
      }
    },
    selectDate(date: Date): void {
      if (this.isDisabled(date)) return;
      this.selectedDate = new Date(date);
      this.showCalendar = false;
      this.$emit("date-selected", this.selectedDate);
    },
    onInput(event: Event): void {
      const value = (event.target as HTMLInputElement).value;
      const parsedDate = this.parseDate(value);
      if (parsedDate && !isNaN(parsedDate.getTime())) {
        this.selectedDate = parsedDate;
        this.$emit("date-selected", this.selectedDate);
      }
    },
    handleClickOutside(event: MouseEvent): void {
      if (this.$refs.datePicker && !(this.$refs.datePicker as HTMLElement).contains(event.target as Node)) {
        this.closeCalendar();
      }
    },
  },
  mounted(): void {
    document.addEventListener("click", this.handleClickOutside);
  },
  beforeDestroy(): void {
    document.removeEventListener("click", this.handleClickOutside);
  },
};
</script>

<style scoped>
.date-picker {
  position: relative;
  display: inline-block;
}

.date-input {
  padding: 5px;
  font-size: 14px;
  background-color: var(--bg-dark);
  color: #e0e0e0;
  border: 1px solid var(--accent);
  border-radius: 4px;
}

.calendar-button {
  margin-left: 5px;
  background: none;
  border: none;
  cursor: pointer;
  color: #e0e0e0;
}

.calendar-popup {
  position: absolute;
  top: 100%;
  left: 0;
  background: var(--bg-dark);
  border: 1px solid var(--accent);
  border-radius: 4px;
  z-index: 10;
  padding: 10px;
  color: #e0e0e0;
}

.calendar-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 10px;
}

.calendar-header button {
  background: none;
  border: none;
  color: #e0e0e0;
  cursor: pointer;
}

.calendar-grid {
  display: grid;
  grid-template-columns: repeat(7, 1fr);
  gap: 5px;
}

.day-header {
  text-align: center;
  font-weight: bold;
  color: #e0e0e0;
}

.day {
  text-align: center;
  padding: 5px;
  cursor: pointer;
  border-radius: 3px;
  background-color: var(--bg);
  color: #e0e0e0;
}

.day:hover {
  background-color: var(--accent2);
}

.day.disabled {
  color: var(--accent);
  background-color: var(--accent2);
  cursor: not-allowed;
}

.day.selected {
  background: var(--accent);
  color: white;
}
</style>
