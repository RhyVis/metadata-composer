<script lang="ts" setup>
import type { UseEdit } from '@/pages/edit/script/useEdit.ts';
import { defaultPassword, useArchiveInfo } from '@/pages/edit/script/useArchiveInfo.ts';
import { ArchiveTypeEnum, ArchiveTypeOptions } from '@/pages/edit/script/define.ts';
import { openPath } from '@tauri-apps/plugin-opener';
import { useGlobalStore } from '@/stores/global.ts';
import { storeToRefs } from 'pinia';
import { useNotify } from '@/composables/useNotify.ts';
import { set } from '@vueuse/core';
import { generateRandomAsciiString } from '@/api/util.ts';
import { Command } from '@/api/cmd.ts';
import { useDarkStyle } from '@/composables/useDarkStyle.ts';

const { isDevMode } = storeToRefs(useGlobalStore());
const { notifyWarning, notifyError } = useNotify();
const { textClass } = useDarkStyle();

const { edit } = defineProps<{
  edit: UseEdit;
}>();
const editArchiveInfo = useArchiveInfo(edit);
const { archiveType, inputPath, inputPassword, flagCreateArchive, doSelect } = editArchiveInfo;

const handlePathOpen = async (path: string | null, resolveArchive: boolean = false) => {
  if (path) {
    try {
      if (resolveArchive) {
        await openPath(await Command.pathResolveArchive(path));
      } else {
        await openPath(path);
      }
    } catch (e) {
      console.error(e);
      notifyError('打开路径失败', e);
    }
  } else {
    notifyWarning('路径未定义', '请先选择一个路径');
  }
};

const handlePassword = () => {
  if (!inputPassword.value) {
    set(inputPassword, defaultPassword);
  } else {
    set(inputPassword, generateRandomAsciiString());
  }
};
</script>

<template>
  <q-card class="q-my-sm" v-if="isDevMode">
    <q-card-section>
      <div class="text-caption">Current Type: {{ archiveType }}</div>
      <div class="text-caption">Input Path: {{ inputPath ?? '`undefined`' }}</div>
      <div class="text-caption">Input Password: {{ inputPassword ?? '`undefined`' }}</div>
    </q-card-section>
  </q-card>
  <q-select
    v-model="archiveType"
    :options="ArchiveTypeOptions"
    emit-value
    hint="存储在磁盘上的方式"
    label="归档类型"
    map-options
    stack-label
  ></q-select>
  <template v-if="archiveType == ArchiveTypeEnum.ArchiveFile">
    <q-field :label="flagCreateArchive ? '源文件路径' : '压缩包路径'" stack-label>
      <div :class="textClass" @click="handlePathOpen(inputPath, true)">
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
          <q-tooltip>
            {{ flagCreateArchive ? '选择文件夹，压缩将不包含文件夹本身' : '选择压缩包' }}
          </q-tooltip>
        </q-btn>
      </template>
    </q-field>
    <q-input v-model="inputPassword" label="密码" stack-label>
      <template #after>
        <q-btn flat icon="password" size="md" square @click="handlePassword" />
      </template>
    </q-input>
  </template>
  <template v-else-if="archiveType == ArchiveTypeEnum.CommonFile">
    <q-field label="源路径" stack-label>
      <div :class="textClass" @click="handlePathOpen(inputPath)">
        {{ inputPath }}
      </div>
      <template #after>
        <q-btn flat icon="insert_drive_file" size="md" square @click="doSelect(true)" />
      </template>
    </q-field>
  </template>
  <template v-else-if="archiveType == ArchiveTypeEnum.Directory">
    <q-field label="源路径" stack-label>
      <div :class="textClass" @click="handlePathOpen(inputPath)">
        {{ inputPath }}
      </div>
      <template #after>
        <q-btn flat icon="folder" size="md" square @click="doSelect(true)" />
      </template>
    </q-field>
  </template>
</template>
