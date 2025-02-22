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
            background: `linear-gradient(to right, ${palette.bg}, ${palette.bgDark})`,
            border: palette.selected ? `2px solid ${palette.accent2}` : 'none'
          }"
          @click="selectPalette(index)"
        >
          <div
            class="palette-circle"
            :style="{
              background: `linear-gradient(to right, ${palette.accent}, ${palette.accentHover})`
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
import { Palette, PaletteData } from "../../types";

export default defineComponent({
  name: "PaletteSelectorModal",
  props: {
    activePaletteIdx: {
      type: Number,
      required: true,
    },
  },
  data() {
    return {
      palettes: [
        {
          name: "Lavender",
          accent: "#3f0f3f",
          accentHover: "#7d1d4d",
          accent2: "#1d7d7d",
          bg: "#1f071f",
          bgDark: "#000000",
          disabledColor: "#4d1d7d",
          selected: false,
        },
        {
          name: "Oceans",
          accent: "#050555",
          accentHover: "#460885",
          accent2: "#088508",
          bg: "#020727",
          bgDark: "#00000d",
          disabledColor: "#b4b4da",
          selected: false,
        },
        {
          name: "Forest",
          accent: "#0e3e3e",
          accentHover: "#174068",
          accent2: "#686817",
          bg: "#061f1f",
          bgDark: "#041414",
          disabledColor: "#d0e0e0",
          selected: false,
        },
        {
          name: "Sunset",
          accent: "#724e0c",
          accentHover: "#a02711",
          accent2: "#11a06e",
          bg: "#201603",
          bgDark: "#010b01",
          disabledColor: "#e2d3b7",
          selected: false,
        },
        {
          name: "Berry",
          accent: "#8b003e",
          accentHover: "#b8004d",
          accent2: "#f95d89",
          bg: "#2c0019",
          bgDark: "#1a000f",
          disabledColor: "#733f5c",
          selected: false,
        },
        {
          name: "Golden",
          accent: "#d4af37",
          accentHover: "#e5c454",
          accent2: "#f9e79f",
          bg: "#3e2b00",
          bgDark: "#2a1e00",
          disabledColor: "#857550",
          selected: false,
        },
        {
          name: "Rose",
          accent: "#c48d87",
          accentHover: "#d8a79f",
          accent2: "#f5cec7",
          bg: "#3b2925",
          bgDark: "#2b1e1b",
          disabledColor: "#806e68",
          selected: false,
        },
        {
          name: "Teal",
          accent: "#00796b",
          accentHover: "#009688",
          accent2: "#80cbc4",
          bg: "#00231d",
          bgDark: "#001512",
          disabledColor: "#4a6663",
          selected: false,
        },
        {
          name: "Twilight",
          accent: "#6d0000",
          accentHover: "#a40000",
          accent2: "#ff6347",
          bg: "#2a0000",
          bgDark: "#140000",
          disabledColor: "#7f3e3e",
          selected: false,
        },
      ],
    };
  },
  computed: {
    selectedPalette() {
      const selectedPalette = this.palettes.find((palette) => palette.selected);
      if (!selectedPalette) {
        return this.palettes[0];
      } else {
        return selectedPalette;
      }
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
        const data = Palette.fromObject(this.selectedPalette);
        const idx = this.palettes.findIndex(palette => palette.name === this.selectedPalette.name);
        const res = new PaletteData(idx, data);
        this.$emit("paletteApplied", res);
        this.closeModal();
      }
    },
    closeModal() {
      this.$emit("close");
    },
  },
  mounted() {
    this.selectPalette(this.activePaletteIdx);
  }
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
  background-color: v-bind("selectedPalette.bg");
  padding: 20px;
  border-radius: 8px;
  width: 90%;
  margin: 2.5%;
  max-width: 400px;
  box-shadow: 0px 4px 10px rgba(0, 0, 0, 0.3);
}

.palette-options {
  display: grid;
  grid-template-columns: 1fr 1fr 1fr;
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
  border: 1px solid v-bind("selectedPalette.accent2");
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

.cancel-btn {
  background-color: v-bind("selectedPalette.accent2");
  color: white;
  border: none;
  padding: 10px 15px;
  border-radius: 4px;
  cursor: pointer;
}

.submit-btn {
  background-color: v-bind("selectedPalette.accent");
  color: white;
  border: none;
  padding: 10px 15px;
  border-radius: 4px;
  cursor: pointer;
}

.cancel-btn:hover {
  background-color: v-bind("selectedPalette.accentHover");
}

.submit-btn:hover {
  background-color: v-bind("selectedPalette.accentHover");
}

.submit-btn:disabled {
  background-color: var(--disabled-color);
  cursor: not-allowed;
}
</style>
