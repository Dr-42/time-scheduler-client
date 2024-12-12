<template>
  <nav class="navbar">
    <div class="left">
      <div class="pancake" @click="toggleAside">
        <menu-icon />
      </div>
    </div>
    <div class="right">
      <div class="settings">
        <cog-icon />
      </div>
    </div>
  </nav>
</template>

<script lang="ts">
import MenuIcon from "vue-material-design-icons/Menu.vue";
import CogIcon from "vue-material-design-icons/Cog.vue";

export default {
  name: "Navbar",
  components: {
    MenuIcon,
    CogIcon,
  },
  methods: {
    toggleAside() {
      this.$emit("toggleAside");
    },
  },
};
</script>

<style scoped>
.navbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin: 0;
  background-color: #0e0e0e;
  color: #e2e2e2;
  width: 100%;
  box-sizing: border-box;
}

.left,
.right {
  display: flex;
  align-items: center;
  padding: 5px;
}

.pancake {
  cursor: pointer;
  display: flex;
  align-items: center;
  padding: 10px;
}

.settings {
  display: flex;
  align-items: center;
  margin-right: 10px;
  padding: 10px;
}
</style>
