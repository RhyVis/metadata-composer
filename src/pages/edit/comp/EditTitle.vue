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
    :rules="[
      (val: string) => !!val || '标题不能为空',
      (val: string) => val.length <= 32 || '标题不能超过32个字符',
      isValidFileSystemString,
    ]"
    clearable
    hint="标题内容"
    label="标题"
    stack-label
  />
</template>
