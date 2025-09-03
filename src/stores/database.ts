import type { Metadata, MetadataOption } from '@/api/types';
import { defineStore } from 'pinia';
import { computed, ref } from 'vue';
import { Command } from '@/api/cmd';
import { extractArchiveSize } from '@/pages/main/script/function';

export const useDatabaseStore = defineStore('database', () => {
  // State
  const items = ref<Metadata[]>([]);

  // Getters
  const size = computed(() => items.value.length);
  const totalFileSize = computed(() =>
    items.value.reduce((sum, item) => sum + extractArchiveSize(item.archive_info), 0),
  );

  // Actions
  const sync = async () => {
    try {
      items.value = await Command.metadataGetAll();
      console.info(`Database synced with ${items.value.length} items.`);
    } catch (e) {
      console.error('Failed to sync database:', e);
      throw e;
    }
  };

  const update = async (item: MetadataOption) => {
    try {
      const createdId = await Command.metadataUpdate(item);
      if (createdId) {
        console.info(`Created item with ID: ${createdId}`);
      } else {
        console.info(`Updated item with ID: ${item.id}`);
      }
      items.value = await Command.metadataGetAll();
    } catch (e) {
      console.error(e);
      throw e;
    }
  };

  const find = (id: string): Metadata | undefined => {
    return items.value.find((item) => item.id == id);
  };

  return {
    // State
    items,
    // Getters
    size,
    totalFileSize,
    // Actions
    sync,
    update,
    find,
  };
});
