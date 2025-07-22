<script lang="ts" setup>
import type { UseEdit } from '@/pages/edit/script/useEdit.ts';
import { ref } from 'vue';
import { set } from '@vueuse/core';
import type { QInput } from 'quasar';
import { useNotify } from '@/composables/useNotify.ts';

const { notifyWarning } = useNotify();

const { edit } = defineProps<{
  edit: UseEdit;
}>();
const { editData, updateField } = edit;

const inputRef = ref<QInput>();
const addCache = ref('');
const resetCache = () => set(addCache, '');

if (!editData.value.alias) {
  console.info('Initializing alias as an empty array');
  editData.value.alias = [];
}

const handleRemoveAlias = (index: number) => {
  const newAlias = [...(editData.value.alias || [])];
  newAlias.splice(index, 1);
  updateField('alias', newAlias);
};
const handleAddAlias = () => {
  const trim = addCache.value.trim();
  if (!trim) return;
  if (checkAliasDuplicate(trim)) {
    notifyWarning(`代称 "${trim}" 已存在`);
    return;
  }
  updateField('alias', [...(editData.value.alias || []), trim]);
  resetCache();
};

const checkAliasDuplicate = (alias: string): boolean => {
  return editData.value.alias?.includes(alias) || false;
};
</script>

<template>
  <q-input
    v-model="addCache"
    :autofocus="false"
    clearable
    hint="回车以添加新标签"
    label="代称"
    lazy-rules
    ref="inputRef"
    stack-label
    @clear="resetCache"
    @keyup.enter="handleAddAlias"
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
