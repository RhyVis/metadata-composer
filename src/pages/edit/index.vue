<script lang="ts" setup>
import { useRouteParams } from '@vueuse/router';
import { inject, onMounted, ref, watch } from 'vue';
import { set } from '@vueuse/core';
import { useRouter } from 'vue-router';
import { LibraryKey, type LibraryStore } from '@/stores/library.ts';
import type { Metadata } from '@/api/types.ts';
import { useEdit } from '@/pages/edit/script/useEdit.ts';
import EditAlias from '@/pages/edit/comp/EditAlias.vue';
import EditTag from '@/pages/edit/comp/EditTag.vue';
import EditArchiveInfo from '@/pages/edit/comp/EditArchiveInfo.vue';
import EditContentInfo from '@/pages/edit/comp/EditContentInfo.vue';
import { Command } from '@/api/cmd.ts';
import { useQuasar } from 'quasar';
import { notifyError, notifySuccess } from '@/api/q-ext.ts';
import EditImage from '@/pages/edit/comp/EditImage.vue';
import { useGlobalStore } from '@/stores/global.ts';
import { storeToRefs } from 'pinia';

const { push } = useRouter();
const { loading, notify } = useQuasar();
const library: LibraryStore = inject(LibraryKey)!;
const { isDevMode } = storeToRefs(useGlobalStore());

const id = ref('');
const data = ref<Metadata | undefined>();
const edit = useEdit(data);
const { editData, updateField, updateData } = edit;

const collectionList = ref<string[]>([]);

const handleUpdate = async () => {
  try {
    loading.show();
    await updateData();
    notify(notifySuccess(`已成功保存 ${editData.value.id || '未命名'}`));
    await push('/');
  } catch (e) {
    console.error(e);
    notify(notifyError('保存失败', e instanceof Error ? e.message : e));
  } finally {
    loading.hide();
  }
};

watch(id, (newID) => {
  if (newID) {
    console.info(`ID updated to: ${newID}`);
    set(data, library.get(newID));
  } else {
    console.warn('ID is empty, switching to NEW mode');
    set(data, undefined);
  }
});

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

  Command.metadataCollectionList()
    .then((value) => set(collectionList, value))
    .catch((e) => {
      console.error('Failed to fetch collection list:', e);
    });
});
</script>

<template>
  <q-page padding>
    <q-card>
      <template v-if="isDevMode">
        <q-card-section>
          <div class="text-subtitle2">Develop Info</div>
          <q-separator />
          <div class="text-caption">ID: {{ id }}</div>
          <div class="text-caption">Data: {{ data ?? '`undefined`' }}</div>
          <div class="text-caption">EditMode: {{ edit.isEditMode }}</div>
          <div class="text-caption">
            EditData:
            <pre>{{ editData }}</pre>
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
          <!-- Collection Select -->
          <q-select
            :model-value="editData.collection"
            :options="collectionList"
            clearable
            label="所属合集"
            new-value-mode="add-unique"
            stack-label
            use-input
            @update:model-value="updateField('collection', $event as string)"
          />
          <!-- Image Input -->
          <EditImage :edit="edit" />
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
          <q-btn icon="save" label="保存" push @click="handleUpdate" />
        </q-btn-group>
      </q-card-actions>
    </q-card>
  </q-page>
</template>
