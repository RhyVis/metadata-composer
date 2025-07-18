import type { ContentInfo, DLContentType, GameData } from '@/api/types.ts';
import type { UseEdit } from '@/pages/edit/script/useEdit.ts';
import { computed, ref, watch } from 'vue';
import { ContentTypeEnum, GameDistributionEnum } from '@/pages/edit/script/define.ts';
import { get } from '@vueuse/core';

export const useContentInfo = (edit: UseEdit) => {
  const { editData, originalData, updateField } = edit;

  const getInitialType = (): string => {
    const data: ContentInfo | null = editData.value.content_info
      ? editData.value.content_info
      : originalData.value?.content_info
        ? originalData.value.content_info
        : null;
    if (!data) return ContentTypeEnum.Undefined;

    function extractType(info: ContentInfo): string {
      if (info === 'Undefined') return ContentTypeEnum.Undefined;
      if ('Game' in info) return ContentTypeEnum.Game;
      return ContentTypeEnum.Undefined;
    }

    return extractType(data);
  };
  const getInitialGameData = (): GameData => {
    const data: ContentInfo | null = editData.value.content_info
      ? editData.value.content_info
      : originalData.value?.content_info
        ? originalData.value.content_info
        : null;
    if (!data || data === 'Undefined') return defaultGameData();

    if ('Game' in data) {
      return {
        ...data.Game,
      };
    }

    return defaultGameData();
  };
  const updateInfo = () => {
    switch (currentType.value) {
      case ContentTypeEnum.Undefined: {
        updateField('content_info', 'Undefined');
        break;
      }
      case ContentTypeEnum.Game: {
        updateField('content_info', {
          Game: get(gInputObject),
        });
      }
    }
  };
  const defaultGameData = (): GameData => ({
    version: '',
    developer: null,
    publisher: null,
    sys_platform: [],
    distribution: 'Unknown',
  });

  const currentType = ref(getInitialType());

  const gInputObject = ref<GameData>(getInitialGameData());
  const gInputDistributionType = computed({
    get: () => {
      const dist = gInputObject.value.distribution;
      if (dist === 'Unknown') return GameDistributionEnum.Unknown;
      if ('Steam' in dist) return GameDistributionEnum.Steam;
      if ('DLSite' in dist) return GameDistributionEnum.DLSite;
      return GameDistributionEnum.Unknown;
    },
    set: (value: string) => {
      switch (value) {
        case GameDistributionEnum.Unknown: {
          gInputObject.value.distribution = GameDistributionEnum.Unknown;
          break;
        }
        case GameDistributionEnum.Steam: {
          gInputObject.value.distribution = { Steam: { app_id: 0 } };
          break;
        }
        case GameDistributionEnum.DLSite: {
          gInputObject.value.distribution = { DLSite: { id: '', content_type: 'Maniax' } };
          break;
        }
      }
    },
  });
  const gInputSteamAppId = computed({
    get: () => {
      const dist = gInputObject.value.distribution;
      return dist !== 'Unknown' && 'Steam' in dist ? dist.Steam.app_id : 0;
    },
    set: (value: number) => {
      if (
        gInputObject.value.distribution !== 'Unknown' &&
        'Steam' in gInputObject.value.distribution
      ) {
        const v = parseInt(String(value), 10);
        if (isNaN(v)) {
          console.warn(`Invalid Steam App ID: ${value}`);
          gInputObject.value.distribution.Steam.app_id = 0;
          return;
        }
        gInputObject.value.distribution.Steam.app_id = parseInt(String(value), 10);
      }
    },
  });
  const gInputDLSiteId = computed({
    get: () => {
      const dist = gInputObject.value.distribution;
      return dist !== 'Unknown' && 'DLSite' in dist ? dist.DLSite.id : '';
    },
    set: (value: string) => {
      if (
        gInputObject.value.distribution !== 'Unknown' &&
        'DLSite' in gInputObject.value.distribution
      ) {
        gInputObject.value.distribution.DLSite.id = value;
      }
    },
  });
  const gInputDLSiteContentType = computed({
    get: () => {
      const dist = gInputObject.value.distribution;
      return dist !== 'Unknown' && 'DLSite' in dist ? dist.DLSite.content_type : 'Maniax';
    },
    set: (value: string) => {
      if (
        gInputObject.value.distribution !== 'Unknown' &&
        'DLSite' in gInputObject.value.distribution
      ) {
        gInputObject.value.distribution.DLSite.content_type = value as DLContentType;
      }
    },
  });

  watch(currentType, (newType) => {
    console.info(`Current Type: ${newType}`);
    updateInfo();
  });
  watch(gInputObject, (newObj) => {
    console.info(`Game Data changed: ${newObj}`);
    if (currentType.value === ContentTypeEnum.Game) {
      updateInfo();
    }
  });

  return {
    currentType,
    gInputObject,
    gInputDistributionType,
    gInputSteamAppId,
    gInputDLSiteId,
    gInputDLSiteContentType,
  };
};
