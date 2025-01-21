<template>
  <div class="modal-backdrop" @click.self="closeModal">
    <div class="modal">
      <h2>Split Block</h2>
      <div class="modal-header">
        <div>Splitting {{ timeblock.title }} of type {{ blockTypes[timeblock.blockTypeId].name}}</div>
        <div>Starting at {{ startTime }} and ending at {{ endTime }}</div>
      </div>
      <form @submit.prevent="submit">
        <div class="form-group">
          <label for="split-time">Split Time</label>
          <time-picker
            :min-time="startTime"
            :max-time="endTime"
            v-model:selected-time="splitTime"
          />
        </div>
        <div class="form-group">
          <label for="before-block-title">Before block</label>
          <input
            type="text"
            id="before-block-name"
            v-model="beforeTitle"
            required
            placeholder="Enter before block name"
          />
          <custom-dropdown
            :options="blockTypes"
            v-model="beforeBlockType"
            placeholder="Select Block Type"
          />
        </div>
        <div class="form-group">
          <label for="after-block-title">After block</label>
          <input
            type="text"
            id="after-block-name"
            v-model="afterTitle"
            required
            placeholder="Enter after block name"
          />
          <custom-dropdown
            :options="blockTypes"
            v-model="afterBlockType"
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
import TimePicker from "../inputs/TimePicker.vue";
import { BlockType, TimeBlock } from "../../types";

type SplitBlockModalData = {
  splitTime: string;
  beforeTitle: string;
  beforeBlockType: number;
  afterTitle: string;
  afterBlockType: number;
};

export default defineComponent({
  name: "SplitBlockModal",
  components: { CustomDropdown, TimePicker },
  emits: ["close", "done"],
  props: {
    timeblock: {
      type: TimeBlock,
      required: true
    },
    blockTypes: {
      type: Array as PropType<BlockType[]>,
      required: true,
    },
  },
  data() {
    return {
      splitTime: "00:00:00",
      beforeTitle: this.timeblock.title,
      beforeBlockType: this.timeblock.blockTypeId,
      afterTitle: this.timeblock.title,
      afterBlockType: this.timeblock.blockTypeId,
    };
  },
  computed: {
    isFormValid(): boolean {
      return (this.splitTime !== this.startTime && this.splitTime !== this.endTime) && this.beforeTitle.trim().length > 0 && this.afterTitle.trim().length > 0;
    },
    startTime(): string {
      const startTime = new Date(this.timeblock.startTime);
      const startHour = startTime.getHours();
      const startMinute = startTime.getMinutes();
      const startSecond = startTime.getSeconds();
      const timeString = startHour.toString().padStart(2, "0") + ":" + startMinute.toString().padStart(2, "0") + ":" + startSecond.toString().padStart(2, "0");
      this.splitTime = timeString;
      return timeString;
    },
    endTime(): string {
      const endTime = new Date(this.timeblock.endTime);
      const endHour = endTime.getHours();
      const endMinute = endTime.getMinutes();
      const endSecond = endTime.getSeconds();
      const timeString = endHour.toString().padStart(2, "0") + ":" + endMinute.toString().padStart(2, "0") + ":" + endSecond.toString().padStart(2, "0");
      return timeString;
    },
  },
  methods: {
    closeModal() {
      this.$emit("close");
    },
    submit() {
      this.$emit("done", {
        splitTime: this.splitTime,
        beforeTitle: this.beforeTitle,
        beforeBlockType: this.beforeBlockType,
        afterTitle: this.afterTitle,
        afterBlockType: this.afterBlockType
      } as SplitBlockModalData);
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
  overflow-y: scroll;
  margin: 2.5%;
  max-width: 400px;
  box-shadow: 0px 4px 10px rgba(0, 0, 0, 0.3);
}

.modal-header {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  padding: 10px;
  background-color: var(--bg-dark);
  border-radius: 8px;
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
