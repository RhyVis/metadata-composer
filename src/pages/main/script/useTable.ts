import type { Metadata } from '@/api/types.ts';
import { storeToRefs } from 'pinia';
import { computed, ref } from 'vue';
import { useLibraryStore } from '@/stores/library.ts';

export const useTable = () => {
  const { items } = storeToRefs(useLibraryStore());

  const searchTag = ref('');
  const searchByRegex = ref(false);

  const searchFunc = computed(() =>
    searchByRegex.value
      ? (a: string, b: string) => !!a.match(b)
      : (a: string, b: string) => a.includes(b),
  );
  const filterFunc = computed(
    () => (rows: Metadata[]) =>
      rows.filter(
        (row) =>
          searchFunc.value(row.title, searchTag.value) ||
          row.alias.some((alias) => searchFunc.value(alias, searchTag.value)) ||
          row.tags.some((tag) => searchFunc.value(tag, searchTag.value)),
      ),
  );

  const rows = computed(() => filterFunc.value(items.value));

  return { searchTag, searchByRegex, rows };
};
