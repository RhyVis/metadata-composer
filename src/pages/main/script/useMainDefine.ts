import type { QSelectOption, QTableColumn } from 'quasar';
import type { Metadata } from '@/api/types';
import { date } from 'quasar';
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { formatBytes } from '@/api/util';
import { FilterTypeEnum } from '@/pages/main/script/define';
import { extractArchiveSize, joinArray } from '@/pages/main/script/function';

export const useMainDefine = () => {
  const { t } = useI18n();

  const colDef = computed((): QTableColumn<Metadata>[] => [
    {
      name: 'title',
      label: t('page.main.table.title'),
      required: true,
      sortable: true,
      align: 'left',
      classes: 'r-no-sel',
      field: 'title',
    },
    {
      name: 'alias',
      label: t('page.main.table.alias'),
      classes: 'r-no-sel',
      field: (row) => joinArray(row.alias),
    },
    {
      name: 'tags',
      label: t('page.main.table.tag'),
      classes: 'r-no-sel',
      field: (row) => joinArray(row.tags),
    },
    {
      name: 'collection',
      label: t('page.main.table.collection'),
      classes: 'r-no-sel',
      field: (row) => (row.collection ? row.collection : t('page.main.table.unknown')),
    },
    {
      name: 'description',
      label: t('page.main.table.description'),
      field: (row) =>
        row.description ? row.description.split('\n') : [t('page.main.table.unknown')],
    },
    {
      name: 'image',
      label: t('page.main.table.image'),
      field: 'image',
    },
    {
      name: 'content_info',
      label: t('page.main.table.content-info'),
      classes: 'r-no-sel',
      field: (row) => row.content_info.type,
    },
    {
      name: 'archive_size',
      label: t('page.main.table.archive-size'),
      classes: 'r-no-sel',
      field: (row) => formatBytes(extractArchiveSize(row.archive_info)),
    },
    {
      name: 'create_time',
      label: t('page.main.table.create-time'),
      sortable: true,
      sortOrder: 'ad',
      sort: (a: string, b: string) => new Date(a).getTime() - new Date(b).getTime(),
      field: (row: Metadata) => date.formatDate(row.create_time, 'YYYY-MM-DD HH:mm:ss'),
    },
    {
      name: 'update_time',
      label: t('page.main.table.update-time'),
      sortable: true,
      sortOrder: 'ad',
      sort: (a: string, b: string) => new Date(a).getTime() - new Date(b).getTime(),
      field: (row: Metadata) => date.formatDate(row.update_time, 'YYYY-MM-DD HH:mm:ss'),
    },
  ]);

  const paginationOptions = computed((): QSelectOption[] =>
    colDef.value
      .filter((col) => col.sortable)
      .map((col) => ({ value: col.name, label: col.label })),
  );

  const filterOptions = computed((): QSelectOption[] => [
    { value: FilterTypeEnum.None, label: t('page.main.option.filter.none') },
    { value: FilterTypeEnum.Deployment, label: t('page.main.option.filter.deployment') },
  ]);

  return {
    colDef,
    paginationOptions,
    filterOptions,
  };
};
