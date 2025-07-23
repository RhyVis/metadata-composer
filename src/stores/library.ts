import type { Metadata, MetadataOption } from '@/api/types.ts';
import { defineStore } from 'pinia';
import { Command } from '@/api/cmd.ts';
import { extractArchiveSize } from '@/pages/main/script/function.ts';

interface LibraryState {
  items: Metadata[];
}

export const useLibraryStore = defineStore('library', {
  state: (): LibraryState => ({
    items: [],
  }),
  getters: {
    size: (state: LibraryState) => state.items.length,
    totalFileSize: (state: LibraryState) =>
      state.items.reduce((sum, item) => sum + extractArchiveSize(item.archive_info), 0),
  },
  actions: {
    async fetch() {
      try {
        this.items = await Command.metadataGetAll();
      } catch (e) {
        console.error(e);
      }
    },
    async update(item: MetadataOption) {
      try {
        const id = await Command.metadataUpdate(item);
        if (id) {
          console.info(`Created item with ID: ${id}`);
        } else {
          console.info(`Updated item with ID: ${item.id}`);
        }
        this.items = await Command.metadataGetAll();
      } catch (e) {
        console.error(e);
        throw e;
      }
    },
    index(id: string): Metadata | undefined {
      return this.items.find((item) => item.id == id);
    },
  },
});
