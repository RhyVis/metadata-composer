import { defineStore } from 'pinia';
import { ref } from 'vue';
import { Command } from '@/api/cmd.ts';

export const useTableStore = defineStore(
  'table',
  () => {
    // State
    const visibleColumns = ref<string[]>(['title', 'alias', 'tags', 'create_time', 'update_time']);
    const pagination = ref({
      sortBy: 'title',
      descending: false,
      rowsPerPage: 6,
    });
    const imageCache = ref<Record<string, string>>({});
    const deploymentCache = ref<string[]>([]);

    // Actions
    async function syncDeploymentCache() {
      try {
        deploymentCache.value = await Command.metadataDeploymentCache();
      } catch (error) {
        console.error('Failed to sync deployment cache:', error);
        throw error;
      }
    }

    return {
      visibleColumns,
      pagination,
      imageCache,
      deploymentCache,
      syncDeploymentCache,
    };
  },
  {
    tauri: {
      filterKeysStrategy: 'pick',
      filterKeys: ['visibleColumns', 'pagination'],
    },
  },
);
