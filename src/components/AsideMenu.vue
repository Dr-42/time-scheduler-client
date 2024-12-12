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
      </ul>
    </div>
    <ul class="footer-links">
      <li @click="closeMenu">
        <router-link to="/about">
          <info-icon class="aside-icon" /> About
        </router-link>
      </li>
      <li @click="closeMenu">
        <router-link to="/license">
          <license-icon class="aside-icon" /> License
        </router-link>
      </li>
    </ul>
  </aside>
</template>

<script>
import HomeIcon from 'vue-material-design-icons/Home.vue';
import HistoryIcon from 'vue-material-design-icons/History.vue';
import AnalysisIcon from 'vue-material-design-icons/ChartLine.vue';
import InfoIcon from 'vue-material-design-icons/Information.vue';
import LicenseIcon from 'vue-material-design-icons/Certificate.vue';

export default {
  name: 'AsideMenu',
  components: {
    HomeIcon,
    HistoryIcon,
    AnalysisIcon,
    InfoIcon,
    LicenseIcon,
  },
  props: {
    isOpen: {
      type: Boolean,
      required: true,
    },
  },
  methods: {
    closeMenu() {
      this.$emit('closeAside');
    },
  },
};
</script>

<style scoped>
.aside {
  background-color: #1e1e1e;
  color: #e2e2e2;
  width: 200px;
  position: fixed;
  top: 0;
  left: -200px;
  bottom: 0;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  padding: 20px 0;
  box-shadow: 2px 0 5px rgba(0, 0, 0, 0.5);
  z-index: 10;
  transition: left 0.3s ease;
}

.title {
  font-size: 20px;
  font-weight: bold;
  margin-bottom: 20px;
  text-align: center;
}

.aside.open {
  left: 0;
}

.aside-icon {
  margin-right: 10px;
  margin-left: 5px;
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
