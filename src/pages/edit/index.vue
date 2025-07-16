<script lang="ts" setup>
import { useRouteParams } from '@vueuse/router';
import { computed, inject, onMounted, ref, watch } from 'vue';
import { get, set } from '@vueuse/core';
import { useRouter } from 'vue-router';
import { LibraryKey, type LibraryStore } from '@/stores/library.ts';
import type { Metadata } from '@/api/types.ts';
import { useEdit } from '@/pages/edit/script/useEdit.ts';
import EditAlias from '@/pages/edit/comp/EditAlias.vue';
import EditTag from '@/pages/edit/comp/EditTag.vue';
import EditArchiveInfo from '@/pages/edit/comp/EditArchiveInfo.vue';
import EditContentInfo from '@/pages/edit/comp/EditContentInfo.vue';

const { push } = useRouter();
const library: LibraryStore = inject(LibraryKey)!;
const dev = computed(() => import.meta.env.DEV);

const id = ref('');
const data = ref<Metadata | undefined>();
const edit = useEdit(data);

const { editData, updateField } = edit;

watch(
  () => id.value,
  () => {
    if (id.value) {
      console.info(`ID updated to: ${id.value}`);
      set(data, library.get(get(id)));
    } else {
      console.warn('ID is empty, switching to NEW mode');
      set(data, undefined);
    }
  },
);

onMounted(() => {
  if (!library) {
    console.error('Library store is not available');
    push('/');
    return;
  }
  const param = useRouteParams('id');
  if (!param.value) {
    console.info('No ID provided, resulting in NEW mode');
  } else if (typeof param.value === 'string') {
    console.info(`Editing item with ID: ${param.value}`);
    set(id, param.value);
  } else {
    console.error('Invalid ID type, expected a string');
    push('/');
  }
});
</script>

<template>
  <q-page padding>
    <q-card>
      <template v-if="dev">
        <q-card-section>
          <div class="text-subtitle2">Develop Info</div>
          <q-separator />
          <div class="text-caption">ID: {{ id }}</div>
          <div class="text-caption">Data: {{ data ?? '`undefined`' }}</div>
          <div class="text-caption">EditMode: {{ edit.isEditMode }}</div>
          <div class="text-caption">
            Edit:
            <pre>{{ edit }}</pre>
          </div>
        </q-card-section>
        <q-separator inset />
      </template>

      <q-card-section>
        <q-form @submit.prevent>
          <!-- Title Input -->
          <q-input
            :model-value="editData.title"
            clearable
            hint="标题内容"
            label="标题"
            stack-label
            @update:model-value="updateField('title', $event as string)"
          />
          <!-- Alias Input -->
          <EditAlias :edit="edit" />
          <!-- Tags Input -->
          <EditTag :edit="edit" />
          <!-- Content Info -->
          <EditContentInfo :edit="edit" />
          <!-- Archive Info -->
          <EditArchiveInfo :edit="edit" />
        </q-form>
      </q-card-section>

      <q-separator inset />

      <q-card-actions>
        <q-space />
        <q-btn-group push>
          <q-btn label="保存" push />
        </q-btn-group>
      </q-card-actions>
    </q-card>
  </q-page>
</template>
