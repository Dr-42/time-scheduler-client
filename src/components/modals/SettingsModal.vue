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
          <div class="password-container">
            <input
              :type="showPassword ? 'text' : 'password'"
              id="password"
              v-model="password"
              required
              placeholder="Enter password"
            />
            <button
              type="button"
              class="toggle-password"
              @click="togglePasswordVisibility"
              aria-label="Toggle password visibility"
            >
              <span v-if="showPassword">
                <eye-off-icon
                  class="eye-icon"
                />
              </span>
              <span v-else>
                <eye-icon
                  class="eye-icon"
                />
              </span>
            </button>
          </div>
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
import EyeIcon from "vue-material-design-icons/Eye.vue";
import EyeOffIcon from "vue-material-design-icons/EyeOff.vue";


export default defineComponent({
  name: "SettingsModal",
  components: {
    EyeIcon,
    EyeOffIcon,
  },
  data() {
    return {
      username: "",
      password: "",
      serverIp: "",
      showPassword: false,
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
    adjustModalForKeyboard(event: FocusEvent) {
      const target = event.target as HTMLElement;
      target.scrollIntoView({ behavior: "smooth", block: "center" });
    },
    togglePasswordVisibility() {
      this.showPassword = !this.showPassword;
    },
  },
  mounted() {
    const inputs = this.$el.querySelectorAll("input");
    inputs.forEach((input: HTMLInputElement) => {
      input.addEventListener("focus", this.adjustModalForKeyboard);
    });
  },
  beforeUnmount() {
    const inputs = this.$el.querySelectorAll("input");
    inputs.forEach((input: HTMLInputElement) => {
      input.removeEventListener("focus", this.adjustModalForKeyboard);
    });
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
  max-height: 80vh;
  overflow-y: auto;
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

.password-container {
  position: relative;
  display: flex;
  align-items: center;
}

input[type="password"],
input[type="text"] {
  flex: 1;
  padding-right: 40px; /* Space for the eye icon */
}

.toggle-password {
  position: absolute;
  right: 10px;
  background: none;
  border: none;
  cursor: pointer;
  font-size: 18px;
  color: white;
  padding: 0;
}

.toggle-password:focus {
  outline: none;
}

.eye-icon {
  margin-right: 5px;
}
</style>
