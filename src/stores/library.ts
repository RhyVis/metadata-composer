import type { InjectionKey } from 'vue';
import type { Metadata, MetadataOption } from '@/api/types.ts';
import { defineStore } from 'pinia';
import { Command } from '@/api/cmd.ts';

interface LibraryState {
  items: Metadata[];
}

export type LibraryStore = ReturnType<typeof useLibraryStore>;

export const LibraryKey: InjectionKey<LibraryStore> = Symbol('library');

export const useLibraryStore = defineStore('library', {
  state: (): LibraryState => ({
    items: [],
  }),
  getters: {
    size: (state: LibraryState) => state.items.length,
  },
  actions: {
    async fetch() {
      try {
        this.items = await Command.libGetAll();
      } catch (e) {
        console.error(e);
      }
    },
    async update(item: MetadataOption) {
      try {
        const id = await Command.libUpdate(item);
        if (id) {
          console.info('Created item with ID:', id);
        } else {
          console.info('Updated item with ID:', item.id);
        }
        this.items = await Command.libGetAll();
      } catch (e) {
        console.error(e);
      }
    },
    get(id: string): Metadata | undefined {
      return this.items.find((item) => item.id === id);
    },
  },
});
