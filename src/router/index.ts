import { createRouter, createWebHistory } from 'vue-router';
import BaseLayout from '@/layout/BaseLayout.vue';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'layout',
      component: BaseLayout,
      children: [
        {
          path: '',
          name: 'main',
          component: () => import('@/pages/main/index.vue'),
        },
        {
          path: 'edit/:id',
          name: 'edit',
          component: () => import('@/pages/edit/index.vue'),
        },
      ],
    },
  ],
});

export default router;
