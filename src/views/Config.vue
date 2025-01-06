<template>
  <div>
    <el-form :model="config" @submit.prevent="updateConfig">
      <el-form-item label="挂载路径">
        <el-input v-model="config.mount_path"></el-input>
      </el-form-item>
      <el-form-item label="扫描间隔(秒)">
        <el-input v-model="config.scan_interval" type="number"></el-input>
      </el-form-item>
      <el-form-item label="启用监控">
        <el-switch v-model="config.enabled"></el-switch>
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click="updateConfig">更新配置</el-button>
      </el-form-item>
    </el-form>
  </div>
</template>

<script>
import axios from 'axios';

export default {
  data() {
    return {
      config: {
        mount_path: '',
        scan_interval: 60,
        enabled: true,
      },
    };
  },
  created() {
    this.fetchConfig();
  },
  methods: {
    async fetchConfig() {
      const response = await axios.get('/api/config');
      this.config = response.data;
    },
    async updateConfig() {
      await axios.post('/api/config', this.config);
      this.$message.success('配置已更新');
    },
  },
};
</script>
