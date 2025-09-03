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
    :hint="$t('page.edit.collection.hint')"
    :label="$t('page.edit.collection.label')"
    :options="collectionList"
    clearable
    new-value-mode="add-unique"
    stack-label
    use-input
  />
</template>
