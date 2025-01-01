<template>
  <div class="modal-backdrop" @click.self="closeModal">
    <div class="modal">
      <h2>Settings</h2>
      <form @submit.prevent="submit">
        <div class="form-group">
          <label for="username">Username</label>
          <input
            type="text"
            id="username"
            v-model="username"
            required
            placeholder="Enter username"
          />
        </div>

        <div class="form-group">
          <label for="password">Password</label>
          <input
            type="password"
            id="password"
            v-model="password"
            required
            placeholder="Enter password"
          />
        </div>

        <div class="form-group">
          <label for="server-ip">Server IP</label>
          <input
            type="text"
            id="server-ip"
            v-model="serverIp"
            required
            placeholder="Enter server IP"
          />
        </div>

        <div class="modal-actions">
          <button type="button" class="cancel-btn" @click="closeModal">Cancel</button>
          <button type="submit" class="submit-btn" :disabled="!isFormValid">Save</button>
        </div>
      </form>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "SettingsModal",
  data() {
    return {
      username: "",
      password: "",
      serverIp: "",
    };
  },
  emits: ["close", "savesettings"],
  computed: {
    isFormValid(): boolean {
      return (
        this.username.trim().length > 0 &&
        this.password.trim().length > 0 &&
        this.serverIp.trim().length > 0
      );
    },
  },
  methods: {
    closeModal() {
      this.$emit("close");
    },
    submit() {
      this.$emit("savesettings", {
        username: this.username,
        password: this.password,
        serverIp: this.serverIp,
      });
    },
  },
});
</script>

<style scoped>
.modal-backdrop {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 20;
}

.modal {
  color: #e2e2e2;
  background-color: var(--bg);
  padding: 20px;
  border-radius: 8px;
  width: 90%;
  margin: 2.5%;
  max-width: 400px;
  box-shadow: 0px 4px 10px rgba(0, 0, 0, 0.3);
}

.form-group {
  margin-bottom: 15px;
  display: flex;
  flex-direction: column;
}

label {
  display: block;
  margin-bottom: 5px;
  font-weight: bold;
}

input {
  padding: 10px;
  margin: 5px;
  border: 1px solid #ddd;
  border-radius: 4px;
  background-color: var(--bg-dark);
  color: white;
  font-size: 16px;
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

.cancel-btn {
  background-color: var(--accent2);
  color: white;
  border: none;
  padding: 10px 15px;
  border-radius: 4px;
  cursor: pointer;
}

.submit-btn {
  background-color: var(--accent);
  color: white;
  border: none;
  padding: 10px 15px;
  border-radius: 4px;
  cursor: pointer;
}

.submit-btn:disabled {
  background-color: var(--disabled-color);
  cursor: not-allowed;
}
</style>
