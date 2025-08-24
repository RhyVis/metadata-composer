<script lang="ts" setup>
import type { UseEdit } from '@/pages/edit/script/useEdit';
import { computed } from 'vue';

const { edit } = defineProps<{
  edit: UseEdit;
}>();
const { editData, updateField } = edit;

const rating = computed({
  get: () => editData.value.rating || 0,
  set: (value) => updateField('rating', value),
});
</script>

<template>
  <q-field :hint="$t('page.edit.rating.hint')" :label="$t('page.edit.rating.label')" stack-label>
    <q-rating class="q-pa-sm" v-model="rating" icon="thumb_up" size="2em" />
  </q-field>
</template>
