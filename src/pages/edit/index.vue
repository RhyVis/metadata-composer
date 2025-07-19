<script lang="ts" setup>
import { useRouteParams } from '@vueuse/router';
import { onMounted, ref } from 'vue';
import { set } from '@vueuse/core';
import { useRouter } from 'vue-router';
import { useLibraryStore } from '@/stores/library.ts';
import type { Metadata } from '@/api/types.ts';
import { useEdit } from '@/pages/edit/script/useEdit.ts';
import EditAlias from '@/pages/edit/comp/EditAlias.vue';
import EditTag from '@/pages/edit/comp/EditTag.vue';
import EditArchiveInfo from '@/pages/edit/comp/EditArchiveInfo.vue';
import EditContentInfo from '@/pages/edit/comp/EditContentInfo.vue';
import { Command } from '@/api/cmd.ts';
import { useQuasar } from 'quasar';
import EditImage from '@/pages/edit/comp/EditImage.vue';
import { useGlobalStore } from '@/stores/global.ts';
import { storeToRefs } from 'pinia';
import { useNotify } from '@/composables/useNotify.ts';
import { getCurrentWindow } from '@tauri-apps/api/window';

const { push } = useRouter();
const { loading } = useQuasar();
const { notifySuccess, notifyError } = useNotify();
const { isDevMode } = storeToRefs(useGlobalStore());
const { get: getById } = useLibraryStore();

const window = getCurrentWindow();

const getData = (): Metadata | undefined => {
  const param = useRouteParams('id');
  if (!param.value) {
    console.info('No ID provided, switching to NEW mode');
    return;
  } else if (typeof param.value !== 'string') {
    console.error(`Invalid ID type, expected a string, but got [${param.value.join(',')}]`);
    push('/');
    return;
  }
  console.info(`Using ID: ${param.value}`);
  return getById(param.value);
};

const id = ref('');
const data = ref<Metadata | undefined>(getData());
const edit = useEdit(data);
const { editData, isEditMode, updateField, updateData, applyPreset } = edit;

const collectionList = ref<string[]>([]);

const handleUpdate = async () => {
  try {
    loading.show();
    const hideWindow = setTimeout(async () => {
      await window.hide();
    }, 2442);
    await updateData();
    clearTimeout(hideWindow);
    await window.show();
    notifySuccess('保存成功', undefined, 1000);
    await push('/');
  } catch (e) {
    console.error(e);
    notifyError('保存失败', e, 1000);
  } finally {
    loading.hide();
    await window.show();
  }
};

onMounted(() => {
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
          <div class="text-caption">EditMode: {{ isEditMode }}</div>
          <div class="text-caption">
            EditData:
            <pre>{{ editData }}</pre>
          </div>
        </q-card-section>
        <q-separator inset />
      </template>

      <q-card-section class="row items-center">
        <div class="text-h6 r-no-sel">
          <template v-if="isEditMode">编辑 {{ editData.title || editData.id }}</template>
          <template v-else>新建元数据</template>
        </div>
        <q-space />
        <q-btn icon="bookmarks" label="预设" outline size="md">
          <q-menu
            anchor="center middle"
            self="top right"
            transition-hide="scale"
            transition-show="scale"
          >
            <q-list style="min-width: 100px">
              <q-item clickable @click="applyPreset('DoujinR18')">
                <q-item-section>DLSite R18 同人</q-item-section>
              </q-item>
            </q-list>
          </q-menu>
        </q-btn>
      </q-card-section>

      <q-separator inset />

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
          <q-input
            :model-value="editData.description"
            autogrow
            clearable
            hint="内容描述"
            label="描述"
            stack-label
            type="textarea"
            @update:model-value="updateField('description', $event as string)"
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
        <q-btn-group outline>
          <q-btn icon="close" label="退出" outline @click="push('/')" />
          <q-btn :label="isEditMode ? '更新' : '保存'" icon="save" outline @click="handleUpdate" />
        </q-btn-group>
      </q-card-actions>
    </q-card>
  </q-page>
</template>
