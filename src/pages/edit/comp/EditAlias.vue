<script lang="ts" setup>
import type { UseEdit } from '@/pages/edit/script/useEdit.ts';
import { ref } from 'vue';
import { set } from '@vueuse/core';
import type { QInput } from 'quasar';
import { useNotify } from '@/hooks/useNotify';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
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
    notifyWarning(t('page.edit.alias.notify.exist-one', [trim]));
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
    :hint="$t('page.edit.alias.hint')"
    :label="$t('page.edit.alias.label')"
    clearable
    lazy-rules
    ref="inputRef"
    stack-label
    @clear="resetCache"
    @keyup.enter="handleAddAlias"
  />
  <div class="q-mt-sm">
    <template v-if="!editData.alias || editData.alias.length === 0">
      <q-chip class="r-no-sel" outline> {{ $t('page.edit.alias.no-alias') }} </q-chip>
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
