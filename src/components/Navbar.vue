<template>
  <nav class="navbar">
    <div class="left">
      <div class="pancake">
        <menu-icon/>
      </div>
    </div>
    <div class="right">
      <div class="settings">
        <cog-icon/>
      </div>
    </div>
  </nav>
</template>

<script>
import MenuIcon from "vue-material-design-icons/Menu.vue";
import CogIcon from "vue-material-design-icons/Cog.vue";

export default {
  name: "Navbar",
  components: {
    MenuIcon,
    CogIcon
  }
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

.left {
  display: flex;
  align-items: center;
  margin-left: 10px;
  padding: 10px;
}

.right {
  display: flex;
  align-items: center;
  margin-right: 10px;
  padding: 10px;
}
</style>
