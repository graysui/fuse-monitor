<template>
  <el-container>
    <el-header>
      <el-menu :default-active="activeMenu" class="el-menu-demo" mode="horizontal">
        <el-menu-item index="/files" @click="navigateTo('/files')">文件列表</el-menu-item>
        <el-menu-item index="/stats" @click="navigateTo('/stats')">监控统计</el-menu-item>
        <el-menu-item index="/config" @click="navigateTo('/config')">配置管理</el-menu-item>
      </el-menu>
    </el-header>
    <el-main>
      <router-view />
    </el-main>
  </el-container>
</template>

<script>
export default {
  data() {
    return {
      activeMenu: '/files'，
    };
  },
  watch: {
    $route(to) {
      this。activeMenu = to.path;
    },
  },
  methods: {
    navigateTo(path) {
      this.$router。push(path);
    },
  },
};
</script>

<style>
.el-menu-demo {
  line-height: 60px;
}
</style>
