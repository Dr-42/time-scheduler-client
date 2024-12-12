<template>
  <div class="modal-backdrop" @click.self="closeModal">
    <div class="modal">
      <h2>Add New Block Type</h2>
      <form @submit.prevent="submit">
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

        <div class="form-group">
          <label for="block-color">Block Color</label>
          <div id="block-color" class="color-picker">
            <Chrome v-model="blockColor.hex" :disable-alpha="true" />
          </div>
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

export default defineComponent({
  name: "AddBlocktypeModal",
  components: { Chrome },
  data() {
    return {
      blockTypeName: "",
      blockColor: {
        hex: "#ffffff",
        rgb: { r: 255, g: 255, b: 255 },
      },
    };
  },
  computed: {
    isFormValid(): boolean {
      return this.blockTypeName.trim().length > 0;
    },
    rgbColor(): string {
      const { r, g, b } = this.blockColor.rgb;
      return `rgb(${r}, ${g}, ${b})`;
    },
  },
  methods: {
    closeModal() {
      this.$emit("close");
    },
    submit() {
      this.$emit("submit", {
        name: this.blockTypeName,
        color: this.blockColor.rgb,
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
  background-color: #2e2e2e;
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
  margin: 5px 0;
  border: 1px solid #ddd;
  border-radius: 4px;
  background-color: #2e2e2e;
  color: white;
  font-size: 16px;
}

.color-picker {
  display: flex;
  flex-direction: row;
  background-color: #2e2e2e;
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
  background-color: #e0e0e0;
  color: #333;
  border: none;
  padding: 10px 15px;
  border-radius: 4px;
  cursor: pointer;
}

.submit-btn {
  background-color: #6200ea;
  color: white;
  border: none;
  padding: 10px 15px;
  border-radius: 4px;
  cursor: pointer;
}

.submit-btn:disabled {
  background-color: #a7a7a7;
  cursor: not-allowed;
}
</style>

<style>
.vc-chrome-body {
  background-color: #2e2e2e;
  color: white;
  padding: auto;
  margin: auto;
}
</style>
