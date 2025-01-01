<template>
  <div class="modal-backdrop" @click.self="closeModal">
    <div class="modal">
      <h2>Add New Block Type</h2>
      <form @submit.prevent="submit">
        <div class="form-group">
          <label for="block-color">Block Color</label>
          <div id="block-color" class="color-picker">
            <!-- TypeScript compiler doesn't understand Proxies for the module Ignore error-->
            <Chrome 
              v-model="blockColor" 
              format="hex"
              :disable-alpha="true"
            />
          </div>
        </div>
        <div class="form-group">
          <label for="block-name">Block Type Name</label>
          <input
            type="text"
            id="block-name"
            v-model="blockTypeName"
            required
            placeholder="Enter block type name"
          />
        </div>
        <div class="modal-actions">
          <button type="button" class="cancel-btn" @click="closeModal">Cancel</button>
          <button type="submit" class="submit-btn" :disabled="!isFormValid">OK</button>
        </div>
      </form>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";
import { Chrome } from "@ckpack/vue-color";
import { Color, NewBlockType } from "../../types";

export default defineComponent({
  name: "AddBlocktypeModal",
  components: { Chrome },
  emits: ["close", "done"],
  data() {
    return {
      blockTypeName: "",
      blockColor: {} as any,
    };
  },
  computed: {
    isFormValid(): boolean {
      return this.blockTypeName.trim().length > 0;
    },
  },
  methods: {
    closeModal() {
      this.$emit("close");
    },
    submit() {
      if (this.blockColor) {
        let r = this.blockColor.rgba.r;
        let g = this.blockColor.rgba.g;
        let b = this.blockColor.rgba.b;
        let col = new Color(r, g, b);
        this.$emit("done", new NewBlockType(this.blockTypeName, col));
      }
    },
    adjustModalForKeyboard(event: FocusEvent) {
      const target = event.target as HTMLElement;
      target.scrollIntoView({ behavior: "smooth", block: "center" });
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

<style>
.vc-chrome-body {
  background-color: var(--bg) !important;
  color: white !important;
  padding: auto !important;
  margin: auto !important;
}
</style>

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
  margin: 5px 0;
  border: 1px solid #ddd;
  border-radius: 4px;
  background-color: var(--bg-dark);
  color: white;
  font-size: 16px;
}

.color-picker {
  display: flex;
  flex-direction: row;
  background-color: var(--bg);
  color: white;
  padding: 10px;
  border-radius: 4px;
  align-items: center;
  justify-content: center;
}

.color-values {
  font-size: 14px;
  color: #ddd;
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
