import type { QTableProps } from 'quasar';
import { defineStore } from 'pinia';

type PaginationW = QTableProps['pagination'] & {
  sortBy: string;
  descending: boolean;
  rowsPerPage: number;
};

interface TableState {
  visibleColumns: string[];
  pagination: PaginationW;
  imageCache: Record<string, string>;
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
  }),
});
