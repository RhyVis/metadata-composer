<script lang="ts" setup>
import { formatBytes } from '@/api/util';
import type { UseOperation } from '@/pages/main/script/useOperation';
import type { UseTable } from '@/pages/main/script/useTable';
import { useDatabaseStore } from '@/stores/database';
import { storeToRefs } from 'pinia';

const { operation, table } = defineProps<{
  operation: UseOperation;
  table: UseTable;
}>();
const { handleReload } = operation;
const { rows } = table;
const { totalFileSize, size } = storeToRefs(useDatabaseStore());
const emit = defineEmits(['new']);
</script>

<template>
  <q-btn-group outline>
    <q-btn outline>
      <div class="row items-center text-subtitle2">{{ formatBytes(totalFileSize) }}</div>
      <q-menu anchor="top middle" self="top middle">
        <q-list separator>
          <q-item v-close-popup clickable @click="handleReload">
            <q-item-section avatar>
              <q-icon name="refresh" />
            </q-item-section>
            <q-item-section>{{ $t('page.main.action.refresh') }}</q-item-section>
          </q-item>
          <q-item v-close-popup clickable @click="emit('new')">
            <q-item-section avatar>
              <q-icon name="add" />
            </q-item-section>
            <q-item-section>{{ $t('page.main.action.add') }}</q-item-section>
          </q-item>
        </q-list>
      </q-menu>
    </q-btn>
    <q-separator vertical />
    <q-btn class="text-subtitle2 r-no-sel" outline>
      {{ size === (rows as []).length ? `${size}` : `${(rows as []).length} / ${size}` }}
    </q-btn>
  </q-btn-group>
</template>
