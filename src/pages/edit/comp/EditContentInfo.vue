<script lang="ts" setup>
import type { UseEdit } from '@/pages/edit/script/useEdit.ts';
import { useContentInfo } from '@/pages/edit/script/useContentInfo.ts';
import { ContentTypeOptions } from '@/pages/edit/script/define.ts';
import { computed } from 'vue';

const dev = computed(() => import.meta.env.DEV);

const { edit } = defineProps<{
  edit: UseEdit;
}>();
const contentInfo = useContentInfo(edit);
const { currentType } = contentInfo;
</script>

<template>
  <q-card class="q-my-sm" v-if="dev">
    <q-card-section>
      <div class="text-caption">Content Type: {{ currentType }}</div>
    </q-card-section>
  </q-card>
  <q-select
    v-model="currentType"
    :options="ContentTypeOptions"
    emit-value
    label="内容类型"
    map-options
    stack-label
  />
</template>
