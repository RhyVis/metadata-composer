<script lang="ts" setup>
import type { UseEdit } from '@/pages/edit/script/useEdit.ts';
import { ref } from 'vue';
import { set } from '@vueuse/core';
import type { QInput } from 'quasar';

const { edit } = defineProps<{
  edit: UseEdit;
}>();
const { editData } = edit;

const addCache = ref('');
const inputRef = ref<QInput>();

const handleRemoveAlias = (index: number) => {
  editData.value.alias?.splice(index, 1);
};
const handleAddAlias = (alias: string) => {
  inputRef.value?.validate();
  if (alias && !editData.value.alias?.includes(alias)) {
    editData.value.alias = [...(editData.value.alias || []), alias];
    set(addCache, '');
  } else {
    console.warn('Alias is empty or already exists');
  }
};

const checkAliasDuplicate = (alias: string): boolean => {
  return editData.value.alias?.includes(alias) || false;
};
</script>

<template>
  <q-input
    v-model="addCache"
    :autofocus="false"
    :rules="[(val) => !checkAliasDuplicate(val) || '此别称已存在']"
    clearable
    label="代称"
    lazy-rules
    placeholder="回车以添加新代称"
    ref="inputRef"
    stack-label
    @clear="addCache = ''"
    @keyup.enter="handleAddAlias(addCache)"
  />
  <div>
    <template v-if="!editData.alias || editData.alias.length === 0">
      <q-chip class="r-no-sel" outline> 无别称 </q-chip>
    </template>
    <template v-else>
      <q-chip
        v-for="(alias, index) in editData.alias"
        :key="index"
        outline
        removable
        @remove="handleRemoveAlias(index)"
      >
        <span class="q-pr-xs">{{ alias }}</span>
      </q-chip>
    </template>
  </div>
</template>
