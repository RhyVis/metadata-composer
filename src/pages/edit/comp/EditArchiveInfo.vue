<script lang="ts" setup>
import type { UseEdit } from '@/pages/edit/script/useEdit.ts';
import { useArchiveInfo } from '@/pages/edit/script/useArchiveInfo.ts';
import { ArchiveTypeEnum, ArchiveTypeOptions } from '@/pages/edit/script/define.ts';
import { openPath } from '@tauri-apps/plugin-opener';
import { useGlobalStore } from '@/stores/global.ts';
import { storeToRefs } from 'pinia';
import { useNotify } from '@/composables/useNotify.ts';

const { isDevMode } = storeToRefs(useGlobalStore());
const { notifyWarning, notifyError } = useNotify();

const { edit } = defineProps<{
  edit: UseEdit;
}>();
const editArchiveInfo = useArchiveInfo(edit);
const { currentType, inputPath, inputPassword, flagCreateArchive, doSelect } = editArchiveInfo;

const openPathW = (path: string | null) => {
  if (path) {
    try {
      openPath(path);
    } catch (e) {
      console.error(e);
      notifyError(
        '打开路径失败',
        `无法打开路径: ${path}. 错误信息: ${e instanceof Error ? e.message : String(e)}`,
      );
    }
  } else {
    notifyWarning('路径未定义', '请先选择一个路径');
  }
};
</script>

<template>
  <q-card class="q-my-sm" v-if="isDevMode">
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
    <q-field :label="flagCreateArchive ? '源文件路径' : '压缩包路径'" stack-label>
      <div @click="openPathW(inputPath)">
        {{ inputPath }}
      </div>
      <template #after>
        <q-checkbox v-model="flagCreateArchive" size="md">
          <q-tooltip>是否创建新的压缩归档</q-tooltip>
        </q-checkbox>
        <q-btn
          :icon="flagCreateArchive ? 'folder' : 'insert_drive_file'"
          flat
          size="md"
          square
          @click="doSelect(flagCreateArchive)"
        >
          <q-tooltip>{{
            flagCreateArchive ? '选择文件夹，压缩将不包含文件夹本身' : '选择压缩包'
          }}</q-tooltip>
        </q-btn>
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
        <q-btn flat icon="insert_drive_file" size="md" square @click="doSelect(true)" />
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
