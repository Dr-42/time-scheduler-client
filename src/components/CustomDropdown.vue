<template>
  <div class="dropdown" ref="dropdown" @click.self="closeDropdown">
    <div class="dropdown-selected" @click="toggleDropdown">
      <div class="left-content">
        <span v-if="selectedOption" class="color-indicator" :style="{ backgroundColor: selectedOption.hexColor }"></span>
        <span>{{ selectedOption ? selectedOption.name : placeholder }}</span>
      </div>
      <span class="dropdown-arrow">▼</span>
    </div>
    <div
      class="dropdown-menu"
      v-if="isOpen"
      :style="{ maxHeight: maxDropdownHeight + 'px', overflowY: 'auto' }"
    >
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

type DropdownValue = number | null | undefined;

export default defineComponent({
  name: "CustomDropdown",
  props: {
    options: {
      type: Array as PropType<Option[]>,
      required: true,
    },
    modelValue: {
      //type: Number,
      // Accept number, null and undefined
      type: Number as PropType<DropdownValue>,
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
      maxDropdownHeight: 200, // Set max height for dropdown menu
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
      if (this.isOpen) {
        document.addEventListener("click", this.handleOutsideClick);
      } else {
        document.removeEventListener("click", this.handleOutsideClick);
      }
    },
    selectOption(option: Option) {
      this.$emit("update:modelValue", option.id);
      this.closeDropdown();
    },
    processColors(options: Option[]) {
      this.processedOptions = options.map(option => ({
        ...option,
        hexColor: `rgb(${option.color.r}, ${option.color.g}, ${option.color.b})`,
      }));
    },
    closeDropdown() {
      this.isOpen = false;
      document.removeEventListener("click", this.handleOutsideClick);
    },
    handleOutsideClick(event: MouseEvent) {
      const dropdown = this.$refs.dropdown as HTMLElement;
      if (dropdown && !dropdown.contains(event.target as Node)) {
        this.closeDropdown();
      }
    },
  },
  mounted() {
    this.processColors(this.options);
  },
  beforeUnmount() {
    document.removeEventListener("click", this.handleOutsideClick);
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

/* Add scroll behavior for the dropdown menu */
.dropdown-menu {
  box-shadow: 0px 4px 6px rgba(0, 0, 0, 0.1);
}
</style>
