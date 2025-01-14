<template>
  <div class="error">
    <div class="error-content">
      {{ errorText }}
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from 'vue';

export default defineComponent({
  name: 'ErrorDisplay',
  props: {
    errorText: {
      type: String,
      required: true,
    },
  },
});

</script>

<style scoped>
.error {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  width: 100%;
  background-color: rgba(30, 0, 0, 0.5);
  z-index: 1000;
}

.error-content {
  padding: 10px;
  width: 80%;
  background-color: #aa0011;
  color: #ffbbbb;
  font-size: 12px;
  border-radius: 5px;
}

</style>
