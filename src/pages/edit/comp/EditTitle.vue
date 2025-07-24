<script lang="ts" setup>
import type { UseEdit } from '@/pages/edit/script/useEdit.ts';
import { isValidFileSystemString } from '@/api/util.ts';
import { computed } from 'vue';

const { edit } = defineProps<{
  edit: UseEdit;
}>();
const { editData, updateField } = edit;

const title = computed({
  get: () => editData.value.title,
  set: (value: string) => updateField('title', value?.trim() ?? ''),
});
</script>

<template>
  <q-input
    v-model="title"
    :hint="$t('page.edit.title.hint')"
    :label="$t('page.edit.title.label')"
    :rules="[
      (val: string) => !!val || $t('page.edit.title.rules.required'),
      (val: string) => val.length <= 32 || $t('page.edit.title.rules.maxLength'),
      isValidFileSystemString,
    ]"
    clearable
    stack-label
  />
</template>
