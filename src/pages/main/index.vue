<script lang="ts" setup>
import { useRouter } from 'vue-router';
import { MainColDef, RowPageSizes } from '@/pages/main/script/define.ts';
import { useTable } from '@/pages/main/script/useTable.ts';
import { storeToRefs } from 'pinia';
import { useOperation } from '@/pages/main/script/useOperation.ts';
import { useTableStore } from './script/useTableStore';
import MainTableTopLeft from '@/pages/main/comp/MainTableTopLeft.vue';
import MainTableTopRight from '@/pages/main/comp/MainTableTopRight.vue';
import MainItemCard from '@/pages/main/comp/MainItemCard.vue';

const { push } = useRouter();

const tableStore = useTableStore();
const { pagination, visibleColumns } = storeToRefs(tableStore);

const table = useTable();
const operation = useOperation();
const { rows } = table;

const handleEdit = (id: string) => {
  console.info(`Editing item with id: ${id}`);
  push(`/edit/${id}`);
};
</script>

<template>
  <q-page padding>
    <div class="text-center">
      <q-table
        v-model:pagination="pagination"
        :columns="MainColDef"
        :rows="rows"
        :rows-per-page-options="RowPageSizes"
        :visible-columns="visibleColumns"
        flat
        grid
        row-key="id"
      >
        <template #top-left>
          <MainTableTopLeft :operation="operation" :table="table" @new="handleEdit('NEW')" />
        </template>
        <template #top-right>
          <MainTableTopRight :table="table" />
        </template>
        <template #item="{ cols, row }">
          <MainItemCard :cols="cols" :operation="operation" :row="row" @edit="handleEdit" />
        </template>
      </q-table>
    </div>
  </q-page>
</template>
