<template>
  <time-picker
    min-time="07:16"
    max-time="13:42"
    v-model="selectedTime"
  />
</template>

<script lang="ts">
import TimePicker from '../inputs/TimePicker.vue';

export default {
  name: "TestBed",
  data() {
    return {
      selectedTime: "07:16"
    }
  },
  components: {
    TimePicker
  },
}
</script>

<style scoped>
</style>
