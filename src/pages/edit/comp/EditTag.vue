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

const handleRemoveTag = (index: number) => {
  editData.value.tags?.splice(index, 1);
};
const handleAddTag = (alias: string) => {
  inputRef.value?.validate();
  if (alias && !editData.value.tags?.includes(alias)) {
    editData.value.tags = [...(editData.value.tags || []), alias];
    set(addCache, '');
  } else {
    console.warn('Tag is empty or already exists');
  }
};

const checkTagDuplicate = (alias: string): boolean => {
  return editData.value.tags?.includes(alias) || false;
};
</script>

<template>
  <q-input
    v-model="addCache"
    :autofocus="false"
    :rules="[(val) => !checkTagDuplicate(val) || '此标签已存在']"
    clearable
    label="标签"
    lazy-rules
    placeholder="回车以添加新标签"
    ref="inputRef"
    stack-label
    @clear="addCache = ''"
    @keyup.enter="handleAddTag(addCache)"
  />
  <div>
    <template v-if="!editData.tags || editData.tags.length === 0">
      <q-chip class="r-no-sel" outline> 无标签 </q-chip>
    </template>
    <template v-else>
      <q-chip
        v-for="(tag, index) in editData.tags"
        :key="index"
        outline
        removable
        @remove="handleRemoveTag(index)"
      >
        <span class="q-pr-xs">{{ tag }}</span>
      </q-chip>
    </template>
  </div>
</template>
