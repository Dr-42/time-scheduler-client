<template>
  <div class="modal-overlay" @click.self="close">
    <div class="modal">
      <h2 class="modal-title">{{ title }}</h2>
      <p class="modal-content">{{ content }}</p>
      <button class="close-btn" @click="close">Close</button>
    </div>
  </div>
</template>

<script>
export default {
  name: 'InfoModal',
  emits: ['close'],
  props: {
    title: {
      type: String,
      required: true,
    },
    content: {
      type: String,
      required: true,
    },
  },
  methods: {
    close() {
      this.$emit('close');
    },
  },
};
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 20;
}

.modal {
  background: var(--bg);
  color: #e2e2e2;
  border-radius: 10px;
  padding: 20px;
  margin: 2.5%;
  width: 80%;
  max-width: 500px;
  box-shadow: 0px 4px 10px rgba(0, 0, 0, 0.3);
  text-align: center;
}

.modal-title {
  font-size: 1.5em;
  margin-bottom: 15px;
}

.modal-content {
  font-size: 1em;
  margin-bottom: 20px;
}

.close-btn {
  background: var(--accent);
  color: white;
  border: none;
  padding: 10px 20px;
  border-radius: 5px;
  cursor: pointer;
}

.close-btn:hover {
  background: var(--accent);
}
</style>
