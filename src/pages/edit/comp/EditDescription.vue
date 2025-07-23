<script lang="ts" setup>
import type { UseEdit } from '@/pages/edit/script/useEdit.ts';
import { computed } from 'vue';

const { edit } = defineProps<{
  edit: UseEdit;
}>();
const { editData, updateField } = edit;

const description = computed({
  get: () => editData.value.description,
  set: (value: string) => updateField('description', value),
});
</script>

<template>
  <q-input
    v-model="description"
    autogrow
    clearable
    hint="内容描述"
    label="描述"
    stack-label
    type="textarea"
  />
</template>
