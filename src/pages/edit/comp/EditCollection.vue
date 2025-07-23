<script lang="ts" setup>
import type { UseEdit } from '@/pages/edit/script/useEdit.ts';
import { computed, onMounted, ref } from 'vue';
import { Command } from '@/api/cmd.ts';
import { set } from '@vueuse/core';

const { edit } = defineProps<{
  edit: UseEdit;
}>();
const { editData, updateField } = edit;

const collectionList = ref<string[]>([]);
const collection = computed({
  get: () => editData.value.collection,
  set: (value: string) => updateField('collection', value?.trim()),
});

onMounted(() => {
  Command.metadataCollectionCache().then(
    (list) => set(collectionList, list),
    (err) => console.error(`Failed to fetch collection list: ${err}`),
  );
});
</script>

<template>
  <q-select
    v-model="collection"
    :options="collectionList"
    clearable
    hint="可通过下拉选项选择已有合集，或输入新合集名称"
    label="所属合集"
    new-value-mode="add-unique"
    stack-label
    use-input
  />
</template>
