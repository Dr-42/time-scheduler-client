<template>
  <div class="dropdown">
    <div class="dropdown-selected" @click="toggleDropdown">
      <div class="left-content">
        <span v-if="selectedOption" class="color-indicator" :style="{ backgroundColor: selectedOption.hexColor }"></span>
        <span>{{ selectedOption ? selectedOption.name : placeholder }}</span>
      </div>
      <span class="dropdown-arrow">▼</span>
    </div>
    <div class="dropdown-menu" v-if="isOpen">
      <div
        v-for="option in processedOptions"
        :key="option.id"
        class="dropdown-item"
        @click="selectOption(option)"
      >
        <span class="color-indicator" :style="{ backgroundColor: option.hexColor }"></span>
        <span>{{ option.name }}</span>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import { defineComponent, PropType } from "vue";

type Option = {
  id: number;
  name: string;
  color: { r: number; g: number; b: number };
  hexColor?: string;
};

export default defineComponent({
  name: "CustomDropdown",
  props: {
    options: {
      type: Array as PropType<Option[]>,
      required: true,
    },
    modelValue: {
      type: Number,
      default: null,
    },
    placeholder: {
      type: String,
      default: "Select an option",
    },
  },
  data() {
    return {
      isOpen: false,
      processedOptions: [] as Option[], // To store options with computed `hexColor`
    };
  },
  computed: {
    selectedOption(): Option | undefined {
      return this.processedOptions.find(option => option.id === this.modelValue);
    },
  },
  watch: {
    options: {
      immediate: true,
      handler(newOptions: Option[]) {
        this.processColors(newOptions);
      },
    },
  },
  methods: {
    toggleDropdown() {
      this.isOpen = !this.isOpen;
    },
    selectOption(option: Option) {
      this.$emit("update:modelValue", option.id);
      this.isOpen = false;
    },
    processColors(options: Option[]) {
      this.processedOptions = options.map(option => ({
        ...option,
        hexColor: `rgb(${option.color.r}, ${option.color.g}, ${option.color.b})`,
      }));
    },
  },
  mounted() {
    this.processColors(this.options);
  },
});
</script>

<style scoped>
.dropdown {
  margin: 5px;
  position: relative;
  display: inline-block;
  width: 100%;
}

.dropdown-selected {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px;
  border: 1px solid #ddd;
  border-radius: 4px;
  cursor: pointer;
  background-color: #2e2e2e;
  color: white;
}

.left-content {
  display: flex;
  align-items: center;
}

.color-indicator {
  width: 20px;
  height: 20px;
  border-radius: 25%;
  margin-right: 10px;
  display: inline-block;
}

.dropdown-arrow {
  margin-left: 10px;
}

.dropdown-menu {
  position: absolute;
  background-color: #1e1e1e;
  color: #e2e2e2;
  width: 100%;
  border: 1px solid #ddd;
  border-radius: 4px;
  margin-top: 5px;
  z-index: 10;
}

.dropdown-item {
  padding: 10px;
  display: flex;
  align-items: center;
  cursor: pointer;
}

.dropdown-item:hover {
  background-color: #2e2e2e;
  color: white;
}
</style>
