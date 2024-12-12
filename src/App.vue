<template>
  <div id="app">
    <navbar @toggleAside="toggleAside" />
    <div class="overlay" v-if="asideOpen" @click="closeAside"></div>
    <aside-menu :isOpen="asideOpen" @closeAside="closeAside" />
    <router-view />
  </div>
</template>

<script>
import Navbar from './components/Navbar.vue';
import AsideMenu from './components/AsideMenu.vue';

export default {
  name: 'App',
  components: {
    Navbar,
    AsideMenu,
  },
  data() {
    return {
      asideOpen: false,
    };
  },
  methods: {
    toggleAside() {
      this.asideOpen = !this.asideOpen;
    },
    closeAside() {
      this.asideOpen = false;
    },
  },
};
</script>

<style scoped>
#app {
  display: flex;
  flex-direction: column;
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
