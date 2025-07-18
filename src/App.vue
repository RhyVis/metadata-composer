<script lang="ts" setup>
import { useLibraryStore } from '@/stores/library.ts';
import { onMounted } from 'vue';
import { useGlobalStore } from '@/stores/global.ts';
import { useNotify } from '@/composables/useNotify.ts';

const { notifyError, notifySuccess } = useNotify();
const { fetch, size } = useLibraryStore();

onMounted(() => {
  fetch().then(
    () => notifySuccess(`成功加载数据库: ${size}`),
    (e) => {
      console.error(e);
      notifyError('加载数据库失败', e);
    },
  );
  useGlobalStore()
    .$tauri.start()
    .then(
      () => console.log('Tauri started successfully'),
      (e) => {
        console.error('Failed to start Tauri:', e);
        notifyError('无法启动同步', e);
      },
    );
});
</script>

<template>
  <router-view />
</template>
