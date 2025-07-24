<script lang="ts" setup>
import type { UseEdit } from '@/pages/edit/script/useEdit.ts';
import { useContentInfo } from '@/pages/edit/script/useContentInfo.ts';
import {
  ContentTypeEnum,
  ContentTypeOptions,
  DLContentTypeOptions,
  GameDistributionEnum,
  GameDistributionOptions,
  GameSysPlatformOptions,
  GameTypeEnum,
  GameTypeOptions,
} from '@/pages/edit/script/define.ts';
import { storeToRefs } from 'pinia';
import { isAsciiNumeric, isNumericOnly, isValidFileSystemString } from '@/api/util.ts';
import { useConfigStore } from '@/stores/config';

const { isDevMode } = storeToRefs(useConfigStore());

const { edit } = defineProps<{
  edit: UseEdit;
}>();

const {
  contentType,
  gInputVersion,
  gInputGameType,
  gInputDeveloper,
  gInputPublisher,
  gInputSysPlatform,
  gInputDistributionType,
  gInputSteamAppId,
  gInputDLSiteId,
  gInputDLSiteContentType,
  gInputOtherName,
  gInputOtherId,
  gViewDLSiteIdPrefix,
  gFetchDLSiteInfo,
  gOpenDLSitePage,
} = useContentInfo(edit);
</script>

<template>
  <q-card class="q-my-sm" v-if="isDevMode">
    <q-card-section>
      <div class="text-caption">{{ $t('develop.display.content-type', [contentType]) }}</div>
    </q-card-section>
  </q-card>
  <q-select
    v-model="contentType"
    :clearable="contentType != ContentTypeEnum.Undefined"
    :label="$t('page.edit.content-info.label')"
    :options="ContentTypeOptions"
    emit-value
    map-options
    stack-label
    @clear="contentType = ContentTypeEnum.Undefined"
  />

  <!-- Type Game Data -->
  <template v-if="contentType == ContentTypeEnum.Game">
    <q-input
      v-model="gInputVersion"
      :hint="$t('page.edit.content-info.game.version.hint')"
      :label="$t('page.edit.content-info.game.version.label')"
      clearable
      stack-label
      @clear="gInputVersion = '1.0.0'"
    />
    <q-select
      v-model="gInputGameType"
      :clearable="gInputGameType != GameTypeEnum.Unspecified"
      :hint="$t('page.edit.content-info.game.type.hint')"
      :label="$t('page.edit.content-info.game.type.label')"
      :options="GameTypeOptions"
      emit-value
      map-options
      stack-label
      @clear="gInputGameType = GameTypeEnum.Unspecified"
    />
    <q-input
      v-model="gInputDeveloper"
      :debounce="$t('page.edit.content-info.game.developer.label')"
      :hint="$t('page.edit.content-info.game.developer.hint')"
      clearable
      stack-label
    />
    <q-input
      v-model="gInputPublisher"
      :hint="$t('page.edit.content-info.game.publisher.hint')"
      :label="$t('page.edit.content-info.game.publisher.label')"
      clearable
      stack-label
    />
    <q-select
      v-model="gInputSysPlatform"
      :hint="$t('page.edit.content-info.game.platform.hint')"
      :label="$t('page.edit.content-info.game.platform.label')"
      :options="GameSysPlatformOptions"
      clearable
      emit-value
      map-options
      multiple
      stack-label
      @clear="gInputSysPlatform = []"
    />
    <q-select
      v-model="gInputDistributionType"
      :hint="$t('page.edit.content-info.game.distribution.hint')"
      :label="$t('page.edit.content-info.game.distribution.label')"
      :options="GameDistributionOptions"
      emit-value
      map-options
      stack-label
    />
    <!-- Dist Steam -->
    <template v-if="gInputDistributionType == GameDistributionEnum.Steam">
      <q-input
        v-model="gInputSteamAppId"
        :hint="$t('page.edit.content-info.game.distribution.steam.hint')"
        :label="$t('page.edit.content-info.game.distribution.steam.label')"
        :rules="[
          (val) => !!val || $t('page.edit.content-info.game.distribution.steam.rule.no-empty'),
          (val) =>
            isNumericOnly(val) ||
            $t('page.edit.content-info.game.distribution.steam.rule.must-num'),
          (val) =>
            val.length <= 16 ||
            $t('page.edit.content-info.game.distribution.steam.rule.char-limit'),
        ]"
        stack-label
      />
    </template>
    <!-- Dist DLSite -->
    <template v-else-if="gInputDistributionType == GameDistributionEnum.DLSite">
      <q-input
        v-model="gInputDLSiteId"
        :hint="$t('page.edit.content-info.game.distribution.dl-site.hint')"
        :label="$t('page.edit.content-info.game.distribution.dl-site.label')"
        :prefix="gViewDLSiteIdPrefix"
        :rules="[
          (val) => !!val || $t('page.edit.content-info.game.distribution.dl-site.rule.no-empty'),
          (val) =>
            isNumericOnly(val) ||
            $t('page.edit.content-info.game.distribution.dl-site.rule.must-num'),
          (val) =>
            val.length <= 16 ||
            $t('page.edit.content-info.game.distribution.dl-site.rule.char-limit'),
        ]"
        stack-label
      >
        <template v-if="gInputDLSiteId" #after>
          <q-btn-group flat>
            <q-btn flat icon="web" round size="md" square @click="gOpenDLSitePage">
              <q-tooltip>
                {{ $t('page.edit.content-info.game.distribution.dl-site.open-page') }}
              </q-tooltip>
            </q-btn>
            <q-btn flat icon="cloud_download" round size="md" square @click="gFetchDLSiteInfo">
              <q-tooltip>
                {{ $t('page.edit.content-info.game.distribution.dl-site.fetch-info') }}
              </q-tooltip>
            </q-btn>
          </q-btn-group>
        </template>
      </q-input>
      <q-select
        v-model="gInputDLSiteContentType"
        :hint="$t('page.edit.content-info.game.distribution.dl-site.content-type.hint')"
        :label="$t('page.edit.content-info.game.content.label')"
        :options="DLContentTypeOptions"
        emit-value
        map-options
        stack-label
      />
    </template>
    <template v-else-if="gInputDistributionType == GameDistributionEnum.Other">
      <q-input
        v-model="gInputOtherName"
        :hint="$t('page.edit.content-info.game.distribution.other.name.hint')"
        :label="$t('page.edit.content-info.game.distribution.other.name.label')"
        :rules="[
          (val) => !!val || $t('page.edit.content-info.game.distribution.other.name.rule.no-empty'),
          (val) =>
            val.length <= 32 ||
            $t('page.edit.content-info.game.distribution.other.name.rule.char-limit'),
          (val) =>
            isAsciiNumeric(val) ||
            $t('page.edit.content-info.game.distribution.other.name.rule.ascii'),
          isValidFileSystemString,
        ]"
        stack-label
      />
      <q-input
        v-model="gInputOtherId"
        :hint="$t('page.edit.content-info.game.distribution.other.id.hint')"
        :label="$t('page.edit.content-info.game.distribution.other.id.label')"
        :rules="[
          (val) => !!val || $t('page.edit.content-info.game.distribution.other.id.rule.no-empty'),
          (val) =>
            isAsciiNumeric(val) ||
            $t('page.edit.content-info.game.distribution.other.id.rule.ascii'),
          (val) =>
            val.length <= 64 ||
            $t('page.edit.content-info.game.distribution.other.id.rule.char-limit'),
          isValidFileSystemString,
        ]"
        stack-label
      />
    </template>
  </template>
</template>
