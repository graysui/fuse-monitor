<template>
  <div>
    <el-table :data="files">
      <el-table-column prop="path" label="文件路径"></el-table-column>
      <el-table-column prop="status" label="状态"></el-table-column>
    </el-table>
  </div>
</template>

<script>
import axios from 'axios';

export default {
  data() {
    return {
      files: [],
    };
  },
  created() {
    this.fetchFiles();
  },
  methods: {
    async fetchFiles() {
      const response = await axios.get('/api/files');
      this.files = response.data;
    },
  },
};
</script>
