<template>
  <div class="history">
    <h1>History</h1>
    <label for="date">Select a date:</label>
    <date-picker
      :maxDate="new Date(maxDate)"
      :minDate="new Date('2021-01-01')"
      :format="'DD-MM-YYYY'"
      @dateSelected="selectedDate = $event"
    />
    <div class="btn">
      <button 
        @click="goToTimecards" 
        :disabled="!isDateValid"
      >
        Get History
      </button>
    </div>
  </div>
</template>

<script lang="ts">
import DatePicker from '../components/inputs/DatePicker.vue';

export default {
  name: "History",
  components: {
    DatePicker
  },
  data() {
    return {
      selectedDate: null,
      isDateInputFocused: false,
    };
  },
  computed: {
    maxDate() {
      const today = new Date();
      const year = today.getFullYear();
      const month = (today.getMonth() + 1).toString().padStart(2, '0');
      const day = today.getDate().toString().padStart(2, '0');
      return `${year}-${month}-${day}`;
    },
    isDateValid() {
      return this.selectedDate !== null;
    },
  },
  methods: {
    goToTimecards() {
      if (this.selectedDate) {
        this.$router.push({ name: "HistoryTimeCards", params: { date: this.selectedDate } });
      }
    },
  },
};
</script>

<style scoped>
.history {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  padding: 20px;
  color: #e2e2e2;
  font-family: Arial, sans-serif;
}

.btn {
  padding: 2%;
  margin-top: 10px;
}

label {
  font-size: 18px;
  margin-bottom: 10px;
}

input {
  font-size: 16px;
  padding: 5px;
  margin-bottom: 10px;
}

button {
  font-size: 16px;
  padding: 10px 20px;
  background-color: #3e3e3e;
  color: #fff;
  border: none;
  border-radius: 5px;
  cursor: pointer;
}

button:disabled {
  background-color: #a9a9a9;
  cursor: not-allowed;
}

button:hover:not(:disabled) {
  background-color: #575757;
}
</style>
