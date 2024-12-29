<template>
  <div>
    <label v-for="blocktype in blocktypes" :key="blocktype.id">
      <input
        type="checkbox"
        :value="blocktype.id"
        v-model="localSelected"
      />
      {{ blocktype.name }}
    </label>
  </div>
</template>

<script lang="ts">
import { BlockType } from '../types';

export default {
  props: {
    blocktypes: { type: Array<BlockType>, default: [] },
    selectedBlockTypes: { type: Array<number>, required: true },
  },
  computed: {
    localSelected: {
      get() {
        return this.selectedBlockTypes;
      },
      set(val: number) {
        this.$emit("update:selectedBlockTypes", val);
      },
    },
  },
};
</script>
