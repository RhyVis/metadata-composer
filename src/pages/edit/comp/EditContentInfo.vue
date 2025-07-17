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
} from '@/pages/edit/script/define.ts';
import { computed } from 'vue';

const dev = computed(() => import.meta.env.DEV);

const { edit } = defineProps<{
  edit: UseEdit;
}>();
const contentInfo = useContentInfo(edit);
const {
  currentType,
  gInputObject,
  gInputDistributionType,
  gInputSteamAppId,
  gInputDLSiteId,
  gInputDLSiteContentType,
} = contentInfo;
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
  <template v-if="currentType == ContentTypeEnum.Game">
    <q-input
      v-model="gInputObject.version"
      clearable
      hint="游戏本体的版本号，默认为1.0.0"
      label="版本"
      stack-label
    />
    <q-input
      v-model="gInputObject.developer"
      clearable
      hint="游戏开发者的名称"
      label="开发者"
      stack-label
    />
    <q-input
      v-model="gInputObject.publisher"
      clearable
      hint="游戏发行商的名称"
      label="发行商"
      stack-label
    />
    <q-select
      v-model="gInputObject.sys_platform"
      :options="GameSysPlatformOptions"
      clearable
      emit-value
      hint="可以运行的平台"
      label="游戏平台"
      map-options
      multiple
      stack-label
    />
    <q-select
      v-model="gInputDistributionType"
      :options="GameDistributionOptions"
      emit-value
      label="游戏发行方式"
      map-options
      stack-label
    />
    <template v-if="gInputDistributionType == GameDistributionEnum.Steam">
      <q-input
        v-model="gInputSteamAppId"
        hint="Steam商店的ID"
        label="Steam App ID"
        stack-label
        type="number"
      />
    </template>
    <template v-else-if="gInputDistributionType == GameDistributionEnum.DLSite">
      <q-input v-model="gInputDLSiteId" hint="DLSite的ID" label="DLSite商店ID" stack-label />
      <q-select
        v-model="gInputDLSiteContentType"
        :options="DLContentTypeOptions"
        emit-value
        label="内容类型"
        map-options
        stack-label
      />
    </template>
  </template>
</template>
