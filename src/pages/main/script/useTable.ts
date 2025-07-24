import type { Metadata } from '@/api/types.ts';
import type { FilterType } from '@/pages/main/script/define.ts';
import { storeToRefs } from 'pinia';
import { computed, ref } from 'vue';
import { FilterTypeEnum } from '@/pages/main/script/define.ts';
import { useTableStore } from '@/pages/main/script/useTableStore';
import { useDatabaseStore } from '@/stores/database';

export const useTable = () => {
  const { items } = storeToRefs(useDatabaseStore());
  const { deploymentCache } = storeToRefs(useTableStore());

  const searchTag = ref('');
  const searchByRegex = ref(false);

  const filterType = ref<FilterType>('None');

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

  const rows = computed(() => {
    switch (filterType.value) {
      case FilterTypeEnum.Deployment: {
        return filterFunc
          .value(items.value)
          .filter((item) => deploymentCache.value.includes(item.id));
      }
      default: {
        return filterFunc.value(items.value);
      }
    }
  });

  return { filterType, searchTag, searchByRegex, rows };
};
