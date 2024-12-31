<template>
  <div id="app">
    <navbar 
      class="navbar"
      @toggleAside="toggleAside"
      @openSettings="currentModal = 'settings'"
      @openPalleteSelector="handleOpenPalleteSelector"
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
  </div>
</template>

<script lang="ts">
import Navbar from './components/subviews/Navbar.vue';
import AsideMenu from './components/subviews/AsideMenu.vue';
import SettingsModal from './components/modals/SettingsModal.vue';
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
  },
  data() {
    return {
      asideOpen: false,
      currentModal: null as string | null,
    };
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

      // Reload the page to apply the new settings
      location.reload();
    },
    handleOpenPalleteSelector() {
      console.log("Opening pallete selector");
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
