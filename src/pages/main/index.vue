<script lang="ts" setup>
import { inject, onMounted } from 'vue';
import { LibraryKey, type LibraryStore } from '@/stores/library.ts';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { storeToRefs } from 'pinia';
import { useRouter } from 'vue-router';
import { MainColDef } from '@/pages/main/script/define.ts';

const library: LibraryStore = inject(LibraryKey)!;
const { items } = storeToRefs(library);
const { push } = useRouter();

const handleCreate = () => {
  push('/edit/new');
};

onMounted(() => {
  if (!library) {
    console.error('Library store is not available');
    getCurrentWindow().close();
  }
});
</script>

<template>
  <q-page padding>
    <div class="text-center">
      <q-table :columns="MainColDef" :rows="items" flat row-key="id">
        <template #top-right>
          <q-btn flat icon="add" round @click="handleCreate" />
        </template>
      </q-table>
    </div>
  </q-page>
</template>
