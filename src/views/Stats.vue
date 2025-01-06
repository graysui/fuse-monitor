<template>
  <div>
    <el-card>
      <div slot="header">
        <span>监控统计信息</span>
      </div>
      <div>总文件数: {{ stats.total_files }}</div>
      <div>文件变更数: {{ stats.files_changed }}</div>
    </el-card>
  </div>
</template>

<script>
import axios from 'axios';

export default {
  data() {
    return {
      stats: {
        total_files: 0,
        files_changed: 0,
      },
    };
  },
  created() {
    this.fetchStats();
  },
  methods: {
    async fetchStats() {
      const response = await axios.get('/api/stats');
      this.stats = response.data;
    },
  },
};
</script>
