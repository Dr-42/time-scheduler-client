<template>
  <aside class="aside" :class="{ open: isOpen }">
    <div>
      <div class="title">Time Scheduler</div>
      <ul class="nav-links">
        <li @click="closeMenu">
          <router-link to="/">
            <home-icon class="aside-icon" /> Home
          </router-link>
        </li>
        <li @click="closeMenu">
          <router-link to="/history">
            <history-icon class="aside-icon" /> History
          </router-link>
        </li>
        <li @click="closeMenu">
          <router-link to="/analysis">
            <analysis-icon class="aside-icon" /> Analysis
          </router-link>
        </li>
        <li @click="closeMenu">
          <router-link to="/test">
            <test-tube-icon class="aside-icon" /> Test
          </router-link>
        </li>
      </ul>
    </div>
    <ul class="footer-links">
      <li @click="openModal('about')">
        <a href="javascript:void(0)">
          <info-icon class="aside-icon" /> About
        </a>
      </li>
      <li @click="openModal('license')">
        <a href="javascript:void(0)">
          <license-icon class="aside-icon" /> License
        </a>
      </li>
    </ul>

    <info-modal
      v-if="modalContent"
      :title="modalContent.title"
      :content="modalContent.body"
      @close="modalContent = null"
    />
  </aside>
</template>

<script lang="ts">
import HomeIcon from 'vue-material-design-icons/Home.vue';
import HistoryIcon from 'vue-material-design-icons/History.vue';
import AnalysisIcon from 'vue-material-design-icons/ChartLine.vue';
import InfoIcon from 'vue-material-design-icons/Information.vue';
import LicenseIcon from 'vue-material-design-icons/Certificate.vue';

import TestTubeIcon from 'vue-material-design-icons/TestTube.vue';

import InfoModal from '../modals/InfoModal.vue';

type ModalContent = {
  title: string;
  body: string;
}

export default {
  name: 'AsideMenu',
  components: {
    InfoModal,

    HomeIcon,
    HistoryIcon,
    AnalysisIcon,
    InfoIcon,
    LicenseIcon,

    TestTubeIcon
  },
  props: {
    isOpen: {
      type: Boolean,
      required: true,
    },
  },
  data() {
    return {
      modalContent: null as ModalContent | null,
    };
  },
  methods: {
    closeMenu() {
      this.$emit('closeAside');
    },
    openModal(type: string) {
      const contentMap: Record<string, { title: string; body: string }> = {
        about: {
          title: 'About',
          body: 'This is the Time Scheduler app, designed to help you manage your time effectively.',
        },
        license: {
          title: 'License',
          body: 'This application is licensed under the BSD-2-Clause license.',
        },
      };
      this.modalContent = contentMap[type];
    },
  },
};
</script>

<style scoped>
.aside {
  background-color: var(--bg);
  color: white;
  width: 200px;
  position: fixed;
  top: 0;
  left: -200px;
  bottom: 0;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  z-index: 10;
  transition: left 0.3s ease;
}

.title {
  font-size: 20px;
  font-weight: bold;
  margin-bottom: 20px;
  text-align: center;
  background-color: var(--accent);
  color: #e2e2e2;
  padding: 20px 0;
}

.aside.open {
  left: 0;
  box-shadow: 2px 0 5px rgba(0, 0, 0, 0.5);
}

.aside-icon {
  margin-right: 10px;
  margin-left: 5px;
  color: #e2e2e2;
}

.nav-links, .footer-links {
  list-style-type: none;
  margin: 0;
  padding: 0;
}

.nav-links li, .footer-links li {
  margin: 10px 0;
}

.nav-links a, .footer-links a {
  color: inherit;
  text-decoration: none;
  display: flex;
  align-items: center;
}

.nav-links a svg, .footer-links a svg {
  margin-right: 10px;
}
</style>
