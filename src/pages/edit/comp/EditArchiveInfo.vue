<script lang="ts" setup>
import type { UseEdit } from '@/pages/edit/script/useEdit.ts';
import { useArchiveInfo } from '@/pages/edit/script/useArchiveInfo.ts';
import { ArchiveTypeEnum, ArchiveTypeOptions } from '@/pages/edit/script/define.ts';
import { computed } from 'vue';
import { openPath } from '@tauri-apps/plugin-opener';
import { useQuasar } from 'quasar';
import { notifyError, notifyWarning } from '@/api/q-ext.ts';

const dev = computed(() => import.meta.env.DEV);
const { notify } = useQuasar();

const { edit } = defineProps<{
  edit: UseEdit;
}>();
const editArchiveInfo = useArchiveInfo(edit);
const { currentType, inputPath, inputPassword, doSelect } = editArchiveInfo;

const openPathW = (path: string | null) => {
  if (path) {
    try {
      openPath(path);
    } catch (e) {
      console.error(e);
      notify(
        notifyError(
          '打开路径失败',
          `无法打开路径: ${path}. 错误信息: ${e instanceof Error ? e.message : String(e)}`,
        ),
      );
    }
  } else {
    notify(notifyWarning('路径未定义', '请先选择一个路径'));
  }
};
</script>

<template>
  <q-card class="q-my-sm" v-if="dev">
    <q-card-section>
      <div class="text-caption">Current Type: {{ currentType }}</div>
      <div class="text-caption">Input Path: {{ inputPath ?? '`undefined`' }}</div>
      <div class="text-caption">Input Password: {{ inputPassword ?? '`undefined`' }}</div>
    </q-card-section>
  </q-card>
  <q-select
    v-model="currentType"
    :options="ArchiveTypeOptions"
    emit-value
    hint="存储在磁盘上的方式"
    label="归档类型"
    map-options
    stack-label
  ></q-select>
  <template v-if="currentType == ArchiveTypeEnum.ArchiveFile">
    <q-field label="源路径" stack-label>
      <div @click="openPathW(inputPath)">
        {{ inputPath }}
      </div>
      <template #after>
        <q-btn flat icon="insert_drive_file" size="md" square @click="doSelect(false)" />
      </template>
    </q-field>
    <q-input v-model="inputPassword" label="密码" stack-label />
  </template>
  <template v-else-if="currentType == ArchiveTypeEnum.CommonFile">
    <q-field label="源路径" stack-label>
      <div @click="openPathW(inputPath)">
        {{ inputPath }}
      </div>
      <template #after>
        <q-btn flat icon="file" size="md" square @click="doSelect(true)" />
      </template>
    </q-field>
  </template>
  <template v-else-if="currentType == ArchiveTypeEnum.Directory">
    <q-field label="源路径" stack-label>
      <div @click="openPathW(inputPath)">
        {{ inputPath }}
      </div>
      <template #after>
        <q-btn flat icon="folder" size="md" square @click="doSelect(true)" />
      </template>
    </q-field>
  </template>
</template>
