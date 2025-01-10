<template>
  <nav class="navbar">
    <div class="left">
      <div class="pancake" @click="toggleAside">
        <menu-icon />
      </div>
    </div>
    <div class="right">
      <div class="pallete" @click="openPalleteSelector">
        <palette-icon />
      </div>
      <div class="settings" @click="openSettings">
        <cog-icon />
      </div>
    </div>
  </nav>
</template>

<script lang="ts">
import MenuIcon from "vue-material-design-icons/Menu.vue";
import CogIcon from "vue-material-design-icons/Cog.vue";
import PaletteIcon from "vue-material-design-icons/Palette.vue";

export default {
  name: "Navbar",
  components: {
    MenuIcon,
    CogIcon,
    PaletteIcon,
  },
  emits: ["toggleAside", "openSettings", "openPalleteSelector"],
  methods: {
    toggleAside() {
      this.$emit("toggleAside");
    },
    openSettings() {
      this.$emit("openSettings");
    },
    openPalleteSelector() {
      this.$emit("openPalleteSelector");
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
  background-color: var(--bg-dark);
  color: #e2e2e2;
  width: 100%;
  box-sizing: border-box;
  z-index: 10;
}

.left,
.right {
  display: flex;
  align-items: center;
  padding: 10px;
  margin: 0;
}

.pancake {
  cursor: pointer;
  display: flex;
  align-items: center;
  padding: 10px;
}

.settings {
  cursor: pointer;
  display: flex;
  align-items: center;
  padding: 10px;
}

.pallete {
  cursor: pointer;
  display: flex;
  align-items: center;
  padding: 10px;
}
</style>
