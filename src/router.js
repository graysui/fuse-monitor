import { createRouter, createWebHistory } from 'vue-router';
import Config from './views/Config.vue';
import Files from './views/Files.vue';
import Stats from './views/Stats.vue';

const routes = [
  { path: '/', redirect: '/files' },
  { path: '/config', component: Config },
  { path: '/files', component: Files },
  { path: '/stats', component: Stats },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
