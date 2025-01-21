<template>
  <div class="modal-backdrop" @click.self="closeModal">
    <div class="modal">
      <h2>Next Block</h2>
      <form @submit.prevent="submit">
        <div class="form-group">
          <label for="block-name">Block Name</label>
          <input
            type="text"
            id="block-name"
            v-model="blockName"
            required
            placeholder="Enter block name"
          />
        </div>

        <div class="form-group">
          <label for="block-type">Block Type</label>
          <CustomDropdown
            :options="blockTypes"
            v-model="blockTypeId"
            placeholder="Select Block Type"
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
import { defineComponent, PropType } from "vue";
import CustomDropdown from "../inputs/CustomDropdown.vue";
import { BlockType, CurrentData } from "../../types";

export default defineComponent({
  name: "NextBlockModal",
  components: { CustomDropdown },
  emits: ["close", "done"],
  props: {
    blockTypes: {
      type: Array as PropType<BlockType[]>,
      required: true,
    },
  },
  data() {
    return {
      blockName: "",
      blockTypeId: undefined as number | undefined,
    };
  },
  computed: {
    isFormValid(): boolean {
      return this.blockName.trim().length > 0 && this.blockTypeId !== null;
    },
  },
  methods: {
    closeModal() {
      this.$emit("close");
    },
    submit() {
      if (this.blockTypeId) {
        this.$emit("done", new CurrentData(this.blockTypeId, this.blockName));
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
  overflow-y: scroll;
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

input,
select {
  padding: 10px;
  margin: 5px;
  border: 1px solid var(--accent);
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

.color-indicator {
  width: 20px;
  height: 20px;
  border-radius: 50%;
  margin-right: 5px;
}
</style>
