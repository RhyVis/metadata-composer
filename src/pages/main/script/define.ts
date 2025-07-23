import type { QSelectOption, QTableColumn } from 'quasar';
import type { Metadata } from '@/api/types.ts';
import { date } from 'quasar';
import { formatBytes } from '@/api/util.ts';
import { extractArchiveSize, joinArray } from '@/pages/main/script/function.ts';

export const MainColDef: QTableColumn<Metadata>[] = [
  {
    name: 'title',
    label: '标题',
    required: true,
    sortable: true,
    align: 'left',
    classes: 'r-no-sel',
    field: 'title',
  },
  {
    name: 'alias',
    label: '别名',
    classes: 'r-no-sel',
    field: (row) => joinArray(row.alias),
  },
  {
    name: 'tags',
    label: '标签',
    classes: 'r-no-sel',
    field: (row) => joinArray(row.tags),
  },
  {
    name: 'collection',
    label: '合集',
    classes: 'r-no-sel',
    field: (row) => (row.collection ? row.collection : '<未提供>'),
  },
  {
    name: 'description',
    label: '描述',
    field: (row) => (row.description ? row.description.split('\n') : ['<未提供>']),
  },
  {
    name: 'image',
    label: '图片',
    field: 'image',
  },
  {
    name: 'content_info',
    label: '内容',
    classes: 'r-no-sel',
    field: (row) => row.content_info.type,
  },
  {
    name: 'archive_size',
    label: '归档大小',
    classes: 'r-no-sel',
    field: (row) => formatBytes(extractArchiveSize(row.archive_info)),
  },
  {
    name: 'create_time',
    label: '创建时间',
    sortable: true,
    sortOrder: 'ad',
    sort: (a: string, b: string) => new Date(a).getTime() - new Date(b).getTime(),
    field: (row: Metadata) => date.formatDate(row.create_time, 'YYYY-MM-DD HH:mm:ss'),
  },
  {
    name: 'update_time',
    label: '更新时间',
    sortable: true,
    sortOrder: 'ad',
    sort: (a: string, b: string) => new Date(a).getTime() - new Date(b).getTime(),
    field: (row: Metadata) => date.formatDate(row.update_time, 'YYYY-MM-DD HH:mm:ss'),
  },
];

export const RowPageSizes = [6, 12, 18, 24, 30, 0];

export const PaginationOptions: QSelectOption[] = MainColDef.filter((col) => col.sortable).map(
  (col) => ({ value: col.name, label: col.label }),
);

export type FilterType = 'None' | 'Deployment';

export const enum FilterTypeEnum {
  None = 'None',
  Deployment = 'Deployment',
}

export const FilterOptions: QSelectOption[] = [
  { value: FilterTypeEnum.None, label: '无' },
  { value: FilterTypeEnum.Deployment, label: '部署' },
];
