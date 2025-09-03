<script lang="ts" setup>
import { useMainDefine } from '@/pages/main/script/useMainDefine';
import type { UseTable } from '@/pages/main/script/useTable';
import { useTableStore } from '@/pages/main/script/useTableStore';
import { storeToRefs } from 'pinia';

const { table } = defineProps<{
  table: UseTable;
}>();
const { filterType, searchTag, searchByRegex } = table;
const { pagination, visibleColumns } = storeToRefs(useTableStore());
const { colDef, paginationOptions, filterOptions } = useMainDefine();
</script>

<template>
  <div class="q-gutter-sm row items-center">
    <q-input v-model="searchTag" :placeholder="$t('page.main.action.search')" dense outlined>
      <template #append>
        <q-icon name="delete" v-if="searchTag" @click="searchTag = ''" />
        <q-icon name="search" v-else />
        <q-checkbox
          v-model="searchByRegex"
          checked-icon="fa-solid fa-code"
          unchecked-icon="fa-solid fa-font"
        >
          <q-tooltip class="r-no-sel">{{
            searchByRegex
              ? $t('page.main.action.search-regex')
              : $t('page.main.action.search-common')
          }}</q-tooltip>
        </q-checkbox>
      </template>
    </q-input>
    <q-select
      v-model="pagination.sortBy"
      :display-value="$t('page.main.action.sort')"
      :options="paginationOptions"
      dense
      emit-value
      map-options
      options-dense
      outlined
    >
      <template #after-options>
        <div class="row items-center r-no-sel q-px-md">
          <div class="q-mr-xs">
            {{ pagination.descending ? $t('general.sort-d') : $t('general.sort-a') }}
          </div>
          <q-checkbox
            v-model="pagination.descending"
            checked-icon="fa-solid fa-sort-down"
            color="primary"
            keep-color
            size="sm"
            unchecked-icon="fa-solid fa-sort-up"
          />
        </div>
      </template>
    </q-select>
    <q-select
      v-model="filterType"
      :display-value="$t('page.main.action.filter')"
      :options="filterOptions"
      dense
      emit-value
      map-options
      options-dense
      outlined
    />
    <q-select
      v-model="visibleColumns"
      :display-value="$t('page.main.action.display-content')"
      :options="colDef.filter((col) => col.name != 'title')"
      dense
      emit-value
      map-options
      multiple
      option-value="name"
      options-dense
      outlined
    />
  </div>
</template>
