<template>
  <div id="app">
    <navbar 
      @toggleAside="toggleAside"
      @openSettings="currentModal = 'settings'"
    />
    <div class="overlay" v-if="asideOpen" @click="closeAside"></div>
    <aside-menu :isOpen="asideOpen" @closeAside="closeAside" />
    <router-view />
    <settings-modal
      v-if="currentModal === 'settings'"
      @close="currentModal = null"
      @savesettings="handleSaveSetting"
    />
  </div>
</template>

<script lang="ts">
import Navbar from './components/Navbar.vue';
import AsideMenu from './components/AsideMenu.vue';
import SettingsModal from './components/SettingsModal.vue';

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
    handleSaveSetting(data: SettingsData) {
      console.log("Settings saved:", data);
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
</style>
