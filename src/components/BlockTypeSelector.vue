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

<script>
export default {
  props: {
    blocktypes: { type: Array, required: true },
    selectedBlockTypes: { type: Array, required: true },
  },
  computed: {
    localSelected: {
      get() {
        return this.selectedBlockTypes;
      },
      set(val) {
        this.$emit("update:selectedBlockTypes", val);
      },
    },
  },
};
</script>
