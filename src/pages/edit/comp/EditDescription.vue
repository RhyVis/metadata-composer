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
    :hint="$t('page.edit.description.hint')"
    :label="$t('page.edit.description.label')"
    autogrow
    clearable
    stack-label
    type="textarea"
  />
</template>
