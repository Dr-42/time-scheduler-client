<template>
  <div class="time-card">
    <div class="left">
      <div class="name">{{ name }}</div>
      <div class="time">{{ startTime }} - {{ endTime }}</div>
    </div>
    <div class="right">
      <div class="duration">{{ duration }}</div>
    </div>
  </div>
</template>

<script lang="ts">
export default {
  props: {
    name: String,
    startTime: String,
    endTime: String,
    duration: String,
    color: String
  }
}
</script>

<style scoped>
.time-card {
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  align-items: center;
  background-color: v-bind(color);
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
