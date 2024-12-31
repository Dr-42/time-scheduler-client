<template>
  <div class="blocktypes-container">
    <div
      v-for="blocktype in blocktypes"
      :key="blocktype.id"
      :class="['blocktype', { selected: localSelected.includes(blocktype.id) }]"
      :style="{ backgroundColor: localSelected.includes(blocktype.id) ? blocktype.color.toString() : '#f0f0f0', color: localSelected.includes(blocktype.id) ? blocktype.color.contrastColor() : '#000' }"
      @click="toggleBlockType(blocktype.id)"
    >
      {{ blocktype.name }}
    </div>
  </div>
</template>

<script lang="ts">
import { BlockType } from '../../types';

export default {
  props: {
    blocktypes: { type: Array<BlockType>, default: () => [] },
    selectedBlockTypes: { type: Array<number>, required: true },
  },
  computed: {
    localSelected: {
      get() {
        return this.selectedBlockTypes;
      },
      set(val: number[]) {
        this.$emit("update:selectedBlockTypes", val);
      },
    },
  },
  methods: {
    toggleBlockType(id: number) {
      const selected = [...this.localSelected];
      const index = selected.indexOf(id);

      if (index === -1) {
        // Add to selection
        selected.push(id);
      } else {
        // Remove from selection
        selected.splice(index, 1);
      }

      this.localSelected = selected;
    },
  },
};
</script>

<style scoped>
.blocktypes-container {
  display: flex;
  flex-wrap: wrap;
  gap: 5px;
  margin: 5px;
}

.blocktype {
  display: inline-block;
  padding: 5px 5px;
  border-radius: 10px;
  border: 1px solid var(--accent);
  cursor: pointer;
  transition: all 0.3s ease;
  text-align: center;
}

.blocktype.selected {
  border-color: transparent;
}

.blocktype:hover {
  opacity: 0.9;
}
</style>
