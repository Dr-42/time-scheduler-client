<template>
  <div class="modal-backdrop" @click.self="closeModal">
    <div class="modal">
      <h2>Choose Theme</h2>
      <div class="palette-options">
        <div
          v-for="(palette, index) in palettes"
          :key="index"
          class="palette-option"
          :style="{
            background: `linear-gradient(to right, ${palette.accent}, ${palette.bg})`,
            border: palette.selected ? '2px solid var(--accent2-color)' : 'none'
          }"
          @click="selectPalette(index)"
        >
          <div
            class="palette-circle"
            :style="{
              background: `linear-gradient(to right, ${palette.accent}, ${palette.bg})`
            }"
          ></div>
          <span>{{ palette.name }}</span>
        </div>
      </div>

      <div class="modal-actions">
        <button type="button" class="cancel-btn" @click="closeModal">Cancel</button>
        <button type="button" class="submit-btn" @click="applyPalette" :disabled="!selectedPalette">
          Apply
        </button>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent } from "vue";

export default defineComponent({
  name: "PaletteSelectorModal",
  data() {
    return {
      palettes: [
        {
          name: "Accent 1",
          accent: "#3e0e3e",
          bg: "#2e2e2e",
          selected: false,
        },
        {
          name: "Accent 2",
          accent: "#6200ea",
          bg: "#121212",
          selected: false,
        },
        {
          name: "Accent 3",
          accent: "#0e3e3e",
          bg: "#0e0e0e",
          selected: false,
        },
      ],
    };
  },
  computed: {
    selectedPalette() {
      return this.palettes.find((palette) => palette.selected);
    },
  },
  methods: {
    selectPalette(index: number) {
      this.palettes.forEach((palette, i) => {
        palette.selected = i === index;
      });
    },
    applyPalette() {
      if (this.selectedPalette) {
        const root = document.documentElement.style;
        root.setProperty("--accent-color", this.selectedPalette.accent);
        root.setProperty("--base-bg", this.selectedPalette.bg);

        this.$emit("paletteApplied", this.selectedPalette);
        this.closeModal();
      }
    },
    closeModal() {
      this.$emit("close");
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
  background-color: var(--base-bg);
  padding: 20px;
  border-radius: 8px;
  width: 90%;
  margin: 2.5%;
  max-width: 400px;
  box-shadow: 0px 4px 10px rgba(0, 0, 0, 0.3);
}

.palette-options {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 15px;
  margin-bottom: 15px;
}

.palette-option {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 10px;
  border-radius: 8px;
  cursor: pointer;
  transition: transform 0.2s ease;
}

.palette-option:hover {
  transform: scale(1.05);
}

.palette-circle {
  width: 50px;
  height: 50px;
  border-radius: 50%;
  margin-bottom: 10px;
  border: 1px solid var(--accent2-color);
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

.cancel-btn {
  background-color: var(--disabled-color);
  color: #333;
  border: none;
  padding: 10px 15px;
  border-radius: 4px;
  cursor: pointer;
}

.submit-btn {
  background-color: var(--accent-color);
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
