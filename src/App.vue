<script lang="ts" setup>
import { useLibraryStore } from '@/stores/library.ts';
import { onMounted, watch } from 'vue';
import { useGlobalStore } from '@/stores/global.ts';
import { useNotify } from '@/composables/useNotify.ts';
import { useConfigStore } from '@/stores/config.ts';
import { useQuasar } from 'quasar';
import { storeToRefs } from 'pinia';
import { get } from '@vueuse/core';
import ContextMenu from '@/layout/ContextMenu.vue';

const { notifyError, notifySuccess } = useNotify();
const { dark } = useQuasar();

const global = useGlobalStore();
const { isDarkMode } = storeToRefs(useGlobalStore());
const { fetch } = useLibraryStore();

onMounted(() => {
  fetch().then(
    () => notifySuccess('成功加载数据库', undefined, 1000),
    (e) => {
      console.error(e);
      notifyError('加载数据库失败', e);
    },
  );
  global.$tauri.start().then(
    () => {
      console.log('Global store started successfully');
      dark.set(get(isDarkMode));
      watch(isDarkMode, (darkMode) => {
        if (darkMode) {
          dark.set(true);
          document.documentElement.classList.add('dark-style');
        } else {
          dark.set(false);
          document.documentElement.classList.remove('dark-style');
        }
      });
    },
    (e) => {
      console.error('Failed to start Tauri:', e);
      notifyError('无法启动同步', e);
    },
  );
  useConfigStore()
    .sync()
    .then(
      () => console.info('Successfully synced config file'),
      (e) => notifyError('配置同步失败', e),
    );
});
</script>

<template>
  <div>
    <RouterView />
    <ContextMenu />
  </div>
</template>
