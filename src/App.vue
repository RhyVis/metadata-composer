<script lang="ts" setup>
import { LibraryKey, useLibraryStore } from '@/stores/library.ts';
import { onMounted, provide } from 'vue';
import { useQuasar } from 'quasar';
import { notifyError, notifySuccess } from '@/api/q-ext.ts';

const { notify } = useQuasar();
const library = useLibraryStore();

provide(LibraryKey, library);

onMounted(async () => {
  try {
    await library.fetch();
    notify(notifySuccess(`成功加载数据库: ${library.size}`));
  } catch (e) {
    console.error(e);
    notify(notifyError('加载数据库失败', e));
  }
});
</script>

<template>
  <router-view />
</template>
