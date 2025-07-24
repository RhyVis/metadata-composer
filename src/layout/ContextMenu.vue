<script lang="ts" setup>
import { useLibraryStore } from '@/stores/library.ts';
import { useNotify } from '@/hooks/useNotify';

const { notifyInfo, notifyError } = useNotify();

const { fetch } = useLibraryStore();

const handleRefresh = async () => {
  try {
    await fetch();
    notifyInfo('库已刷新', undefined, 1000);
  } catch (e) {
    console.error('Failed to refresh library:', e);
    notifyError('刷新库失败', e);
  }
};
</script>

<template>
  <q-menu context-menu touch-position>
    <q-list dense style="max-width: 120px">
      <q-item v-close-popup clickable to="/">
        <q-item-section avatar>
          <q-icon name="home" />
        </q-item-section>
        <q-item-section>主页</q-item-section>
      </q-item>
      <q-item v-close-popup clickable to="/config">
        <q-item-section avatar>
          <q-icon name="settings" />
        </q-item-section>
        <q-item-section>设置</q-item-section>
      </q-item>
      <q-separator />
      <q-item v-close-popup clickable @click="handleRefresh">
        <q-item-section avatar>
          <q-icon name="refresh" />
        </q-item-section>
        <q-item-section>刷新</q-item-section>
      </q-item>
    </q-list>
  </q-menu>
</template>
