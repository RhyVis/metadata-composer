import type { QForm } from 'quasar';
import type { Ref } from 'vue';
import type { CompressionInfoPayload } from '@/api/event.ts';
import type { Metadata, MetadataOption } from '@/api/types.ts';
import type { EditPreset } from '@/pages/edit/script/define.ts';
import type { UnlistenFn } from '@tauri-apps/api/event';
import { cloneDeep } from 'lodash-es';
import { useQuasar } from 'quasar';
import { computed, ref } from 'vue';
import { useI18n } from 'vue-i18n';
import { truncateString } from '@/api/util.ts';
import { useNotify } from '@/hooks/useNotify';
import { useTray } from '@/hooks/useTray';
import {
  ContentTypeEnum,
  DLContentTypeEnum,
  GameDistributionEnum,
  GameSysPlatformEnum,
} from '@/pages/edit/script/define.ts';
import { useDatabaseStore } from '@/stores/database';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { sendNotification } from '@tauri-apps/plugin-notification';
import { get, useToggle } from '@vueuse/core';

export type UseEdit = ReturnType<typeof useEdit>;

export type MaybeMetadata = Metadata | undefined;

type EditableField = Exclude<keyof MetadataOption, 'id'>;

export const useEdit = (id: Ref<string>, formRef: Ref<QForm>) => {
  const { t } = useI18n();
  const { update, find } = useDatabaseStore();
  const { loading } = useQuasar();
  const { notifySuccess, notifyError, notifyWarning } = useNotify();
  const { tooltip } = useTray();

  const window = getCurrentWindow();

  const initData = () => find(get(id));
  const initialData = ref<MaybeMetadata>(initData());

  const isEditMode = computed(() => !!initialData.value?.id);
  const [everEdited, setEverEdited] = useToggle(false);
  const [pageLock, setPageLock] = useToggle(false);

  const mapEditData = (): MetadataOption => {
    const copy: MaybeMetadata = cloneDeep(initialData.value);
    return {
      id: copy?.id ?? null,
      title: copy?.title ?? null,
      alias: copy?.alias ?? null,
      tags: copy?.tags ?? null,
      collection: copy?.collection ?? null,
      description: copy?.description ?? null,
      image: copy?.image ?? null,
      rating: copy?.rating ?? null,
      content_info: copy?.content_info ?? null,
      archive_info: copy?.archive_info ?? null,
      flag_create_archive: false,
    };
  };

  const editData = ref<MetadataOption>(mapEditData());

  const updateField = <K extends EditableField>(field: K, value: MetadataOption[K]) => {
    editData.value[field] = value;
    setEverEdited(true);
  };
  const clearField = <K extends EditableField>(field: K) => {
    editData.value[field] = null as never;
    setEverEdited(true);
  };

  const displayName = computed(
    () => editData.value.title || editData.value.id || t('general.unknown'),
  );
  const updatingMsg = computed(
    () =>
      `${isEditMode.value ? t('page.edit.loading.update') : t('page.edit.loading.create')} ${displayName.value} ${t('general.data')}...`,
  );

  const updateData = async (): Promise<boolean> => {
    if (!(await validate())) {
      notifyWarning(t('page.edit.notify.validation-fail'));
      return false;
    }

    let eventHandle: UnlistenFn | undefined;
    try {
      const msg = get(updatingMsg);
      loading.show({
        message: msg,
      });
      await tooltip(msg);

      listen<CompressionInfoPayload>('compression_progress', (event) => {
        const progress = event.payload[0];
        const fileCount = event.payload[1];
        const currentFile = truncateString(event.payload[2], 25);
        loading.show({
          message: t('page.edit.loading.compress-info', [msg, progress, fileCount, currentFile]),
          html: true,
        });
        tooltip(
          t('page.edit.tooltip.compress-info', [msg, progress, fileCount, currentFile]),
        ).catch(console.error);
      }).then(
        (handle) => (eventHandle = handle),
        (error) => console.error(`Failed to listen to compression_progress: ${error}`),
      );

      setPageLock(true);
      await update(get(editData));
      setEverEdited(false);

      const successMsg = isEditMode.value
        ? t('page.edit.notify.update.success')
        : t('page.edit.notify.create.success');
      notifySuccess(successMsg, undefined, 1000);
      if (!(await window.isFocused())) sendNotification(successMsg);

      return true;
    } catch (e) {
      console.error(e);
      notifyError(
        isEditMode.value ? t('page.edit.notify.update.fail') : t('page.edit.notify.create.fail'),
        e,
        1000,
      );
      return false;
    } finally {
      loading.hide();
      eventHandle?.();
      await tooltip();
      setPageLock(false);
    }
  };

  const applyPreset = (preset: EditPreset) => {
    switch (preset) {
      case 'DoujinR18': {
        console.info('Applying preset: DoujinR18');
        updateField('content_info', {
          type: ContentTypeEnum.Game,
          data: {
            version: '1.0.0',
            game_type: 'RPG',
            developer: null,
            publisher: null,
            sys_platform: [GameSysPlatformEnum.Windows],
            distribution: {
              type: GameDistributionEnum.DLSite,
              data: {
                id: '',
                content_type: DLContentTypeEnum.DoujinR18,
              },
            },
          },
        });
        break;
      }
      default: {
        console.warn(`Unknown preset: ${preset}`);
      }
    }
  };

  const validate = async () => {
    try {
      return await formRef.value.validate();
    } catch (e) {
      console.error(e);
      notifyError(t('page.edit.notify.validation-fail'), e);
      return false;
    }
  };

  return {
    isEditMode,
    everEdited,
    pageLock,
    editData,
    updateField,
    clearField,
    updateData,
    applyPreset,
    validate,
  };
};
