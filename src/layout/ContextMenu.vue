<script lang="ts" setup>
import { useNotify } from '@/hooks/useNotify';
import { useI18n } from 'vue-i18n';
import { useDatabaseStore } from '@/stores/database';

const { t } = useI18n();
const { notifyInfo, notifyError } = useNotify();

const { sync } = useDatabaseStore();

const handleRefresh = async () => {
  try {
    await sync();
    notifyInfo(t('database.refresh-success'), undefined, 1000);
  } catch (e) {
    console.error('Failed to refresh database:', e);
    notifyError(t('database.refresh-fail'), e);
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
        <q-item-section>{{ $t('layout.context-menu.home') }}</q-item-section>
      </q-item>
      <q-item v-close-popup clickable to="/config">
        <q-item-section avatar>
          <q-icon name="settings" />
        </q-item-section>
        <q-item-section>{{ $t('layout.context-menu.settings') }}</q-item-section>
      </q-item>
      <q-separator />
      <q-item v-close-popup clickable @click="handleRefresh">
        <q-item-section avatar>
          <q-icon name="refresh" />
        </q-item-section>
        <q-item-section>{{ $t('layout.context-menu.refresh') }}</q-item-section>
      </q-item>
    </q-list>
  </q-menu>
</template>
