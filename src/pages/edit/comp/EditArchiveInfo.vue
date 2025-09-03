<script lang="ts" setup>
import type { UseEdit } from '@/pages/edit/script/useEdit.ts';
import { defaultPassword, useArchiveInfo } from '@/pages/edit/script/useArchiveInfo.ts';
import { ArchiveTypeEnum } from '@/pages/edit/script/define.ts';
import { openPath } from '@tauri-apps/plugin-opener';
import { storeToRefs } from 'pinia';
import { useNotify } from '@/hooks/useNotify';
import { set } from '@vueuse/core';
import { generateRandomAsciiString } from '@/api/util.ts';
import { Command } from '@/api/cmd.ts';
import { useDarkStyle } from '@/hooks/useDarkStyle';
import { useConfigStore } from '@/stores/config';
import { useI18n } from 'vue-i18n';
import { useEditDefine } from '@/pages/edit/script/useEditDefine';

const { t } = useI18n();
const { isDevMode } = storeToRefs(useConfigStore());
const { notifyWarning, notifyError } = useNotify();
const { textClass } = useDarkStyle();

const { edit } = defineProps<{
  edit: UseEdit;
}>();
const editArchiveInfo = useArchiveInfo(edit);
const { archiveType, inputPath, inputPassword, flagCreateArchive, doSelect } = editArchiveInfo;

const { archiveTypeOptions } = useEditDefine();

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
      notifyError(t('notify.open-path.fail'), e);
    }
  } else {
    notifyWarning(t('notify.open-path.no-path'), t('notify.open-path.choose-one-please'));
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
      <div class="text-caption">{{ $t('develop.display.current-type', [archiveType]) }}</div>
      <div class="text-caption">{{ $t('develop.display.path', [inputPath || '`undefined`']) }}</div>
      <div class="text-caption">
        {{ $t('develop.display.password', [inputPassword || '`undefined`']) }}
      </div>
    </q-card-section>
  </q-card>
  <q-select
    v-model="archiveType"
    :hint="$t('page.edit.archive-info.hint')"
    :label="$t('page.edit.archive-info.label')"
    :options="archiveTypeOptions"
    clearable
    emit-value
    map-options
    stack-label
    @clear="archiveType = ArchiveTypeEnum.None"
  />
  <template v-if="archiveType == ArchiveTypeEnum.ArchiveFile">
    <q-field
      :label="
        flagCreateArchive
          ? $t('page.edit.archive-info.archive-file.label-source')
          : $t('page.edit.archive-info.archive-file.label-archive')
      "
      stack-label
    >
      <div :class="textClass" @click="handlePathOpen(inputPath, true)">
        {{
          inputPath ||
          (flagCreateArchive
            ? $t('page.edit.archive-info.archive-file.no-folder')
            : $t('page.edit.archive-info.archive-file.no-archive'))
        }}
      </div>
      <template #after>
        <q-checkbox v-model="flagCreateArchive" size="md">
          <q-tooltip>{{ $t('page.edit.archive-info.archive-file.tooltip') }}</q-tooltip>
        </q-checkbox>
        <q-btn
          :icon="flagCreateArchive ? 'folder' : 'folder_zip'"
          flat
          round
          size="md"
          @click="doSelect(flagCreateArchive)"
        >
          <q-tooltip>
            {{
              flagCreateArchive
                ? $t('page.edit.archive-info.archive-file.tooltip-choose-folder')
                : $t('page.edit.archive-info.archive-file.tooltip-choose-archive')
            }}
          </q-tooltip>
        </q-btn>
      </template>
    </q-field>
    <q-input v-model="inputPassword" :label="$t('general.password')" stack-label>
      <template #after>
        <q-btn flat icon="password" round size="md" @click="handlePassword">
          <q-tooltip>{{
            inputPassword
              ? $t('page.edit.archive-info.archive-file.fill-rand-pw')
              : $t('page.edit.archive-info.archive-file.fill-default-pw')
          }}</q-tooltip>
        </q-btn>
      </template>
    </q-input>
  </template>
  <template v-else-if="archiveType == ArchiveTypeEnum.CommonFile">
    <q-field :label="$t('page.edit.archive-info.common-file.label')" stack-label>
      <div :class="textClass" @click="handlePathOpen(inputPath)">
        {{ inputPath || $t('general.file-none') }}
      </div>
      <template #after>
        <q-btn flat icon="insert_drive_file" round size="md" @click="doSelect(true)" />
      </template>
    </q-field>
  </template>
  <template v-else-if="archiveType == ArchiveTypeEnum.Directory">
    <q-field :label="$t('page.edit.archive-info.directory.label')" stack-label>
      <div :class="textClass" @click="handlePathOpen(inputPath)">
        {{ inputPath || $t('general.folder-none') }}
      </div>
      <template #after>
        <q-btn flat icon="folder" round size="md" @click="doSelect(true)" />
      </template>
    </q-field>
  </template>
</template>
