import type { ContentInfo, DLContentType, GameData, GameDistribution } from '@/api/types.ts';
import type { UseEdit } from '@/pages/edit/script/useEdit.ts';
import { useQuasar } from 'quasar';
import { computed } from 'vue';
import { Command } from '@/api/cmd.ts';
import { isNumericOnly } from '@/api/util.ts';
import { useNotify } from '@/hooks/useNotify';
import { DLContentTypeEnum } from '@/pages/edit/script/define.ts';
import { openUrl } from '@tauri-apps/plugin-opener';
import { get, set } from '@vueuse/core';

const defaultContentInfo = (): ContentInfo => ({
  type: 'Undefined',
});

const defaultGameData = (): GameData => ({
  version: '1.0.0',
  game_type: 'Unspecified',
  developer: null,
  publisher: null,
  sys_platform: [],
  distribution: {
    type: 'Unknown',
  },
});

export const useContentInfo = (edit: UseEdit) => {
  const { editData, updateField } = edit;
  const { loading } = useQuasar();
  const { notifyWarning, notifyError } = useNotify();

  if (!editData.value.content_info) {
    console.info('Initializing content_info with default values');
    editData.value.content_info = defaultContentInfo();
  }

  const contentInfo = computed<ContentInfo>({
    get: () => editData.value.content_info!,
    set: (val: ContentInfo) => updateField('content_info', val),
  });

  // Type Game

  const isTypeGameDLSite = computed(
    () =>
      contentInfo.value.type === 'Game' && contentInfo.value.data.distribution.type === 'DLSite',
  );

  const gInputVersion = computed({
    get: () => (contentInfo.value.type === 'Game' ? contentInfo.value.data.version : ''),
    set: (val: GameData['version']) => {
      if (contentInfo.value.type === 'Game') {
        updateField('content_info', {
          type: 'Game',
          data: {
            ...contentInfo.value.data,
            version: val?.trim() || '1.0.0',
          },
        });
      } else {
        console.warn('Attempted to set version on non-Game content type');
      }
    },
  });
  const gInputGameType = computed({
    get: () =>
      contentInfo.value.type === 'Game' ? contentInfo.value.data.game_type : 'Unspecified',
    set: (val: GameData['game_type']) => {
      if (contentInfo.value.type === 'Game') {
        updateField('content_info', {
          type: 'Game',
          data: {
            ...contentInfo.value.data,
            game_type: val,
          },
        });
      } else {
        console.warn('Attempted to set game type on non-Game content type');
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
            developer: val ? val.trim() : null,
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
            publisher: val ? val.trim() : null,
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
        : '',
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
        : DLContentTypeEnum.Doujin,
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
  const gInputOtherName = computed({
    get: () =>
      contentInfo.value.type === 'Game' && contentInfo.value.data.distribution.type === 'Other'
        ? contentInfo.value.data.distribution.data.name
        : '',
    set: (val: string) => {
      if (
        contentInfo.value.type === 'Game' &&
        contentInfo.value.data.distribution.type === 'Other'
      ) {
        updateField('content_info', {
          type: 'Game',
          data: {
            ...contentInfo.value.data,
            distribution: {
              type: 'Other',
              data: { ...contentInfo.value.data.distribution.data, name: val?.trim() },
            },
          },
        });
      } else {
        console.warn('Attempted to set Other name on non-Game or non-Other content type');
      }
    },
  });
  const gInputOtherId = computed({
    get: () =>
      contentInfo.value.type === 'Game' && contentInfo.value.data.distribution.type === 'Other'
        ? contentInfo.value.data.distribution.data.id
        : '',
    set: (val: string) => {
      if (
        contentInfo.value.type === 'Game' &&
        contentInfo.value.data.distribution.type === 'Other'
      ) {
        updateField('content_info', {
          type: 'Game',
          data: {
            ...contentInfo.value.data,
            distribution: {
              type: 'Other',
              data: { ...contentInfo.value.data.distribution.data, id: val?.trim() },
            },
          },
        });
      } else {
        console.warn('Attempted to set Other ID on non-Game or non-Other content type');
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
          case 'Other': {
            updateField('content_info', {
              type: 'Game',
              data: {
                ...contentInfo.value.data,
                distribution: {
                  type: val,
                  data: {
                    name: '',
                    id: '',
                  },
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
  const gViewDLSiteUrlSeg = computed(() => {
    if (
      contentInfo.value.type === 'Game' &&
      contentInfo.value.data.distribution.type === 'DLSite'
    ) {
      switch (gInputDLSiteContentType.value) {
        case DLContentTypeEnum.Doujin:
          return 'home';
        case DLContentTypeEnum.Comics:
          return 'comic';
        case DLContentTypeEnum.PcGames:
          return 'soft';
        case DLContentTypeEnum.SmartphoneGames:
          return 'app';
        case DLContentTypeEnum.DoujinR18:
          return 'maniax';
        case DLContentTypeEnum.ComicsR18:
          return 'books';
        case DLContentTypeEnum.HGames:
          return 'pro';
        case DLContentTypeEnum.SmartphoneGamesR18:
          return 'appx';
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

  const gFetchDLSiteInfo = async () => {
    if (!isTypeGameDLSite.value) {
      console.warn('Attempted to fetch DLSite info on non-DLSite content type');
      return;
    }

    const id = gInputDLSiteId.value.trim();
    if (!id) {
      notifyWarning('DLSite ID 不能为空，请检查输入');
      return;
    }
    if (!isNumericOnly(id)) {
      notifyWarning('DLSite ID 应为不包含前缀的数字，请检查输入');
      return;
    }

    try {
      loading.show({
        message: `正在获取 ${gViewDLSiteIdPrefix.value}${id} 的信息...`,
      });
      const data = await Command.utilDlFetchInfo({
        id,
        content_type: gInputDLSiteContentType.value,
      });

      if (data.title && !editData.value.title) {
        updateField('title', data.title);
      }
      if (data.circle) {
        if (!gInputDeveloper.value) set(gInputDeveloper, data.circle);
        if (!gInputPublisher.value) set(gInputPublisher, data.circle);
      }
      if (data.tags && data.tags.length > 0) {
        const tags = new Set(editData.value.tags);
        data.tags.forEach((tag) => tags.add(tag));
        updateField('tags', Array.from(tags));
      }
      if (data.description && !editData.value.description) {
        updateField('description', data.description.join('\n'));
      }
    } catch (e) {
      console.error('Failed to fetch DLSite info:', e);
      notifyError('获取 DLSite 信息失败，请稍后再试');
    } finally {
      loading.hide();
    }
  };
  const gOpenDLSitePage = async () => {
    if (!isTypeGameDLSite.value || !gInputDLSiteId.value) {
      console.warn('Attempted to open DLSite page on non-DLSite content type or without ID');
      notifyWarning('请先设置 DLSite ID，然后再尝试打开页面');
      return;
    }
    const url = `https://www.dlsite.com/${get(gViewDLSiteUrlSeg)}/work/=/product_id/${get(gViewDLSiteIdPrefix)}${get(gInputDLSiteId)}.html`;
    try {
      await openUrl(url);
    } catch (e) {
      console.error('Failed to open DLSite page:', e);
      notifyError('打开 DLSite 页面失败，请检查网络连接或稍后再试', url);
    }
  };

  return {
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
    gViewDLSiteUrlSeg,
    gFetchDLSiteInfo,
    gOpenDLSitePage,
  };
};
