import type { ContentInfo, DLContentType, GameData, GameDistribution } from '@/api/types.ts';
import type { UseEdit } from '@/pages/edit/script/useEdit.ts';
import { computed } from 'vue';
import { isNumericOnly } from '@/api/util.ts';
import { DLContentTypeEnum } from '@/pages/edit/script/define.ts';

const defaultContentInfo = (): ContentInfo => ({
  type: 'Undefined',
});

const defaultGameData = (): GameData => ({
  version: '',
  developer: null,
  publisher: null,
  sys_platform: [],
  distribution: {
    type: 'Unknown',
  },
});

export const useContentInfo = (edit: UseEdit) => {
  const { editData, updateField } = edit;

  if (!editData.value.content_info) {
    console.info('Initializing content_info with default values');
    editData.value.content_info = defaultContentInfo();
  }

  const contentInfo = computed<ContentInfo>({
    get: () => editData.value.content_info!,
    set: (val: ContentInfo) => (editData.value.content_info = val),
  });

  // Type Game

  const gInputVersion = computed({
    get: () => (contentInfo.value.type === 'Game' ? contentInfo.value.data.version : ''),
    set: (val: GameData['version']) => {
      if (contentInfo.value.type === 'Game') {
        updateField('content_info', {
          type: 'Game',
          data: {
            ...contentInfo.value.data,
            version: val,
          },
        });
      } else {
        console.warn('Attempted to set version on non-Game content type');
      }
    },
  });
  const gInputDeveloper = computed({
    get: () => (contentInfo.value.type === 'Game' ? contentInfo.value.data.developer : null),
    set: (val: GameData['developer']) => {
      if (contentInfo.value.type === 'Game') {
        updateField('content_info', {
          type: 'Game',
          data: {
            ...contentInfo.value.data,
            developer: val,
          },
        });
      } else {
        console.warn('Attempted to set developer on non-Game content type');
      }
    },
  });
  const gInputPublisher = computed({
    get: () => (contentInfo.value.type === 'Game' ? contentInfo.value.data.publisher : null),
    set: (val: GameData['publisher']) => {
      if (contentInfo.value.type === 'Game') {
        updateField('content_info', {
          type: 'Game',
          data: {
            ...contentInfo.value.data,
            publisher: val,
          },
        });
      } else {
        console.warn('Attempted to set publisher on non-Game content type');
      }
    },
  });
  const gInputSysPlatform = computed({
    get: () => (contentInfo.value.type === 'Game' ? contentInfo.value.data.sys_platform : []),
    set: (val: GameData['sys_platform']) => {
      if (contentInfo.value.type === 'Game') {
        updateField('content_info', {
          type: 'Game',
          data: {
            ...contentInfo.value.data,
            sys_platform: val,
          },
        });
      } else {
        console.warn('Attempted to set sys_platform on non-Game content type');
      }
    },
  });
  const gInputSteamAppId = computed({
    get: () =>
      contentInfo.value.type === 'Game' && contentInfo.value.data.distribution.type === 'Steam'
        ? contentInfo.value.data.distribution.data.app_id
        : '0',
    set: (val: string) => {
      if (
        contentInfo.value.type === 'Game' &&
        contentInfo.value.data.distribution.type === 'Steam'
      ) {
        const trim = val.trim();
        if (!isNumericOnly(trim)) {
          console.warn('App ID should be numeric only, please check the input');
          return;
        }
        updateField('content_info', {
          type: 'Game',
          data: {
            ...contentInfo.value.data,
            distribution: {
              type: 'Steam',
              data: { app_id: trim },
            },
          },
        });
      } else {
        console.warn('Attempted to set Steam App ID on non-Game or non-Steam content type');
      }
    },
  });

  const gInputDLSiteId = computed({
    get: () =>
      contentInfo.value.type === 'Game' && contentInfo.value.data.distribution.type === 'DLSite'
        ? contentInfo.value.data.distribution.data.id
        : '',
    set: (val: string) => {
      if (
        contentInfo.value.type === 'Game' &&
        contentInfo.value.data.distribution.type === 'DLSite'
      ) {
        const trim = val.trim();
        if (!isNumericOnly(trim)) {
          console.warn('DLSite ID should be numeric only, please check the input');
          return;
        }
        updateField('content_info', {
          type: 'Game',
          data: {
            ...contentInfo.value.data,
            distribution: {
              type: 'DLSite',
              data: {
                ...contentInfo.value.data.distribution.data,
                id: trim,
              },
            },
          },
        });
      } else {
        console.warn('Attempted to set DLSite ID on non-Game or non-DLSite content type');
      }
    },
  });
  const gInputDLSiteContentType = computed({
    get: () =>
      contentInfo.value.type === 'Game' && contentInfo.value.data.distribution.type === 'DLSite'
        ? contentInfo.value.data.distribution.data.content_type
        : 'Unknown',
    set: (val: DLContentType) => {
      if (
        contentInfo.value.type === 'Game' &&
        contentInfo.value.data.distribution.type === 'DLSite'
      ) {
        updateField('content_info', {
          type: 'Game',
          data: {
            ...contentInfo.value.data,
            distribution: {
              type: 'DLSite',
              data: { ...contentInfo.value.data.distribution.data, content_type: val },
            },
          },
        });
      } else {
        console.warn('Attempted to set DLSite content type on non-Game or non-DLSite content type');
      }
    },
  });
  const gInputDistributionType = computed({
    get: () =>
      contentInfo.value.type === 'Game' ? contentInfo.value.data.distribution.type : 'Unknown',
    set: (val: GameDistribution['type']) => {
      if (contentInfo.value.type === 'Game') {
        switch (val) {
          case 'Unknown': {
            updateField('content_info', {
              type: 'Game',
              data: {
                ...contentInfo.value.data,
                distribution: { type: val },
              },
            });
            break;
          }
          case 'Steam': {
            updateField('content_info', {
              type: 'Game',
              data: {
                ...contentInfo.value.data,
                distribution: { type: val, data: { app_id: '0' } },
              },
            });
            break;
          }
          case 'DLSite': {
            updateField('content_info', {
              type: 'Game',
              data: {
                ...contentInfo.value.data,
                distribution: {
                  type: val,
                  data: { id: '', content_type: DLContentTypeEnum.Doujin },
                },
              },
            });
            break;
          }
          default: {
            console.warn(`Unsupported distribution type change to ${val}`);
          }
        }
      } else {
        console.warn('Attempted to set distribution type on non-Game content type');
      }
    },
  });

  const gViewDLSiteIdPrefix = computed(() => {
    if (
      contentInfo.value.type === 'Game' &&
      contentInfo.value.data.distribution.type === 'DLSite'
    ) {
      switch (gInputDLSiteContentType.value) {
        case DLContentTypeEnum.Doujin:
        case DLContentTypeEnum.DoujinR18:
        case DLContentTypeEnum.SmartphoneGames:
        case DLContentTypeEnum.SmartphoneGamesR18: {
          return 'RJ';
        }
        case DLContentTypeEnum.Comics:
        case DLContentTypeEnum.ComicsR18: {
          return 'BJ';
        }
        case DLContentTypeEnum.PcGames:
        case DLContentTypeEnum.HGames: {
          return 'VJ';
        }
      }
    }
    return undefined;
  });

  const contentType = computed<ContentInfo['type']>({
    get: () => editData.value.content_info!.type,
    set: (val: ContentInfo['type']) => {
      switch (val) {
        case 'Game': {
          updateField('content_info', {
            type: 'Game',
            data: defaultGameData(),
          });
          break;
        }
        default: {
          console.warn(`Unsupported content type change to ${val}`);
          updateField('content_info', defaultContentInfo());
        }
      }
    },
  });

  return {
    contentType,
    gInputVersion,
    gInputDeveloper,
    gInputPublisher,
    gInputSysPlatform,
    gInputDistributionType,
    gInputSteamAppId,
    gInputDLSiteId,
    gInputDLSiteContentType,
    gViewDLSiteIdPrefix,
  };
};
