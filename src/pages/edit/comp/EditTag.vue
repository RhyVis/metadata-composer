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

if (!editData.value.tags) {
  console.info('Initializing tags as an empty array');
  editData.value.tags = [];
}

const handleRemoveTag = (index: number) => {
  const newTags = [...(editData.value.tags || [])];
  newTags.splice(index, 1);
  updateField('tags', newTags);
};
const handleAddTag = (alias: string) => {
  const trim = alias.trim();
  if (!trim) return;

  const tags = trim.split(/[\s，,；;|]+/).filter(Boolean);
  const duplicatedTags = [];
  const toAddTags = [];
  console.info(`Adding tags: ${tags.join(', ')}`);

  let added = false;
  for (const tag of tags) {
    if (checkTagDuplicate(tag)) {
      duplicatedTags.push(tag);
    } else {
      toAddTags.push(tag);
      added = true;
    }
  }

  if (added) {
    updateField('tags', [...(editData.value.tags || []), ...toAddTags]);
    resetCache();
  }
  if (duplicatedTags.length > 0) {
    notifyWarning(`标签 ${duplicatedTags.join(', ')} 已存在`);
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
    clearable
    hint="回车以添加新标签"
    label="标签"
    lazy-rules
    ref="inputRef"
    stack-label
    @clear="resetCache"
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
