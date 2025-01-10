<template>
  <div class="spinner">
    <div class="circle"></div>
  </div>
</template>

<script lang="ts">
export default {
  name: "LoadingSpinner",
};
</script>

<style scoped>
.spinner {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
}

.circle {
  border: 4px solid var(--bg-dark);
  border-top: 4px solid var(--accent);
  border-radius: 50%;
  width: 40px;
  height: 40px;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% {
    transform: rotate(0deg);
  }
  100% {
    transform: rotate(360deg);
  }
}
</style>
