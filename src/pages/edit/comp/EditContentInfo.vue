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
import { useGlobalStore } from '@/stores/global.ts';
import { isNumericOnly, isValidFileSystemString } from '@/api/util.ts';

const { isDevMode } = storeToRefs(useGlobalStore());

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
      <div class="text-caption">Content Type: {{ contentType }}</div>
    </q-card-section>
  </q-card>
  <q-select
    v-model="contentType"
    :clearable="contentType != ContentTypeEnum.Undefined"
    :options="ContentTypeOptions"
    emit-value
    label="内容类型"
    map-options
    stack-label
    @clear="contentType = ContentTypeEnum.Undefined"
  />

  <!-- Type Game Data -->
  <template v-if="contentType == ContentTypeEnum.Game">
    <q-input
      v-model="gInputVersion"
      clearable
      hint="游戏本体的版本号，默认为1.0.0"
      label="版本"
      stack-label
      @clear="gInputVersion = '1.0.0'"
    />
    <q-select
      v-model="gInputGameType"
      :clearable="gInputGameType != GameTypeEnum.Unspecified"
      :options="GameTypeOptions"
      emit-value
      hint="游戏内容类型"
      label="类型"
      map-options
      stack-label
      @clear="gInputGameType = GameTypeEnum.Unspecified"
    />
    <q-input
      v-model="gInputDeveloper"
      clearable
      hint="游戏开发者的名称"
      label="开发者"
      stack-label
    />
    <q-input
      v-model="gInputPublisher"
      clearable
      hint="游戏发行商的名称"
      label="发行商"
      stack-label
    />
    <q-select
      v-model="gInputSysPlatform"
      :options="GameSysPlatformOptions"
      clearable
      emit-value
      hint="可以运行的平台"
      label="游戏平台"
      map-options
      multiple
      stack-label
      @clear="gInputSysPlatform = []"
    />
    <q-select
      v-model="gInputDistributionType"
      :options="GameDistributionOptions"
      emit-value
      label="游戏发行方式"
      map-options
      stack-label
    />
    <!-- Dist Steam -->
    <template v-if="gInputDistributionType == GameDistributionEnum.Steam">
      <q-input
        v-model="gInputSteamAppId"
        :rules="[
          (val) => !!val || 'Steam App ID不能为空',
          (val) => isNumericOnly(val) || 'Steam App ID必须是数字',
          (val) => val.length <= 16 || 'Steam App ID长度不能超过16个字符',
        ]"
        hint="Steam商店的ID"
        label="Steam App ID"
        stack-label
      />
    </template>
    <!-- Dist DLSite -->
    <template v-else-if="gInputDistributionType == GameDistributionEnum.DLSite">
      <q-input
        v-model="gInputDLSiteId"
        :prefix="gViewDLSiteIdPrefix"
        :rules="[
          (val) => !!val || 'DLSite商店ID不能为空',
          (val) => isNumericOnly(val) || 'DLSite商店ID必须是数字',
          (val) => val.length <= 16 || 'DLSite商店ID长度不能超过16个字符',
        ]"
        hint="DLSite的ID"
        label="DLSite商店ID"
        stack-label
      >
        <template v-if="gInputDLSiteId" #after>
          <q-btn-group flat>
            <q-btn flat icon="web" round size="md" square @click="gOpenDLSitePage">
              <q-tooltip>打开网址</q-tooltip>
            </q-btn>
            <q-btn flat icon="cloud_download" round size="md" square @click="gFetchDLSiteInfo">
              <q-tooltip>根据ID从DLSite官网获取信息</q-tooltip>
            </q-btn>
          </q-btn-group>
        </template>
      </q-input>
      <q-select
        v-model="gInputDLSiteContentType"
        :options="DLContentTypeOptions"
        emit-value
        label="内容类型"
        map-options
        stack-label
      />
    </template>
    <template v-else-if="gInputDistributionType == GameDistributionEnum.Other">
      <q-input
        v-model="gInputOtherName"
        :rules="[
          (val) => !!val || '名称不能为空',
          (val) => val.length <= 32 || '名称长度不能超过32个字符',
          isValidFileSystemString,
        ]"
        hint="其他发行方式的名称"
        label="名称"
        stack-label
      />
      <q-input
        v-model="gInputOtherId"
        :rules="[
          (val) => !!val || 'ID不能为空',
          (val) => val.length <= 64 || 'ID长度不能超过64个字符',
          isValidFileSystemString,
        ]"
        hint="其他发行方式的ID"
        label="ID"
        stack-label
      />
    </template>
  </template>
</template>
