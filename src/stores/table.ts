import type { QTableProps } from 'quasar';
import { defineStore } from 'pinia';
import { Command } from '@/api/cmd.ts';

type PaginationW = QTableProps['pagination'] & {
  sortBy: string;
  descending: boolean;
  rowsPerPage: number;
};

interface TableState {
  visibleColumns: string[];
  pagination: PaginationW;
  imageCache: Record<string, string>;
  deploymentCache: string[];
}

export const useTableStore = defineStore('table', {
  state: (): TableState => ({
    visibleColumns: ['title', 'alias', 'tags', 'create_time', 'update_time'],
    pagination: {
      sortBy: 'title',
      descending: false,
      rowsPerPage: 6,
    },
    imageCache: {},
    deploymentCache: [],
  }),
  actions: {
    syncDeploymentCache() {
      Command.metadataDeploymentCache().then(
        (value) => (this.deploymentCache = value),
        (error) => console.error('Failed to sync deployment cache:', error),
      );
    },
  },
  tauri: {
    filterKeysStrategy: 'pick',
    filterKeys: ['visibleColumns', 'pagination'],
  },
});
