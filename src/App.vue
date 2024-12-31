<template>
  <div id="app">
    <navbar 
      class="navbar"
      @toggleAside="toggleAside"
      @openSettings="currentModal = 'settings'"
      @openPalleteSelector="currentModal = 'paletteSelector'"
    />
    <error-display
      v-if="error" 
      :errorText="errorText" 
    />
    <div class="overlay" v-if="asideOpen" @click="closeAside"></div>
    <aside-menu :isOpen="asideOpen" @closeAside="closeAside" />
    <div class="content">
      <router-view />
    </div>
    <settings-modal
      v-if="currentModal === 'settings'"
      @close="currentModal = null"
      @savesettings="handleSaveSetting"
    />
    <palette-selector-modal
      v-if="currentModal === 'paletteSelector'"
      @close="currentModal = null"
      @paletteApplied="applyPalette"
    />
  </div>
</template>

<script lang="ts">
import Navbar from './components/subviews/Navbar.vue';
import AsideMenu from './components/subviews/AsideMenu.vue';
import SettingsModal from './components/modals/SettingsModal.vue';
import PaletteSelectorModal from './components/modals/PaletteSelectorModal.vue';
import ErrorDisplay from './components/subviews/ErrorDisplay.vue';
import { Palette } from './types';
import { invoke } from '@tauri-apps/api/core';

type SettingsData = {
  username: string;
  password: string;
  serverIp: string;
}

export default {
  name: 'App',
  components: {
    Navbar,
    AsideMenu,
    SettingsModal,
    PaletteSelectorModal,
    ErrorDisplay
  },
  data() {
    return {
      asideOpen: false,
      currentModal: null as string | null,
      error: false,
      errorText: "",
    };
  },
  async mounted() {
    try {
      const paletteJson = await invoke("get_palette");
      const palette = Palette.fromJson(paletteJson);
      const root = document.documentElement.style;
      root.setProperty("--accent", palette.accent);
      root.setProperty("--accent2", palette.accent2);
      root.setProperty("--bg", palette.bg);
      root.setProperty("--bg-dark", palette.bgDark);
      root.setProperty("--accent-hover", palette.accentHover);
      root.setProperty("--disabled-color", palette.disabledColor);
    } catch (e) {
      console.error(e);
      this.error = true;
      this.errorText = e as string;
    }
  },
  methods: {
    toggleAside() {
      this.asideOpen = !this.asideOpen;
    },
    closeAside() {
      this.asideOpen = false;
    },
    async handleSaveSetting(data: SettingsData) {
      await invoke('save_meta', {
        "username": data.username,
        "password": data.password,
        "serverIp": data.serverIp,
      });
      this.currentModal = null;
      location.reload();
    },
    async applyPalette(palette: Palette) {
      await invoke('save_palette', { palette: palette.toJson() });
      const root = document.documentElement.style;
      root.setProperty("--accent", palette.accent);
      root.setProperty("--accent2", palette.accent2);
      root.setProperty("--bg", palette.bg);
      root.setProperty("--bg-dark", palette.bgDark);
      root.setProperty("--accent-hover", palette.accentHover);
      this.currentModal = null;
    },
  },
};
</script>

<style scoped>
#app {
  display: flex;
  flex-direction: column;
  overflow-x: hidden;
  height: 100vh;
  width: 100vw;
  margin: 0;
  padding: 0;
}

.overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.5);
  z-index: 9;
}

.navbar {
  height: 10vh;
}
</style>
