import type { Ref } from 'vue';
import type { Metadata, MetadataOption } from '@/api/types.ts';
import type { EditPreset } from '@/pages/edit/script/define.ts';
import type { UnlistenFn } from '@tauri-apps/api/event';
import { cloneDeep } from 'lodash-es';
import { useQuasar } from 'quasar';
import { computed, ref } from 'vue';
import { useNotify } from '@/composables/useNotify.ts';
import { useTray } from '@/composables/useTray.ts';
import { useLibraryStore } from '@/stores/library.ts';
import { listen } from '@tauri-apps/api/event';
import { get } from '@vueuse/core';

export type UseEdit = ReturnType<typeof useEdit>;

export type MaybeMetadata = Metadata | undefined;

export const useEdit = (initialData: Ref<MaybeMetadata>) => {
  const { update } = useLibraryStore();
  const { loading } = useQuasar();
  const { notifySuccess, notifyError } = useNotify();
  const { tooltip } = useTray();

  const isEditMode = computed(() => !!initialData.value?.id);

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
      content_info: copy?.content_info ?? null,
      archive_info: copy?.archive_info ?? null,
      flag_create_archive: false,
    };
  };

  const editData = ref<MetadataOption>(mapEditData());

  const updateField = <K extends keyof MetadataOption>(field: K, value: MetadataOption[K]) => {
    editData.value[field] = value;
  };
  const clearField = <K extends keyof MetadataOption>(field: K) => {
    editData.value[field] = null as never;
  };

  const updatingMsg = computed(
    () =>
      `正在${isEditMode.value ? '更新 ' : '创建 '}${editData.value.title || editData.value.id || '未知'} 数据...`,
  );

  const updateData = async (): Promise<boolean> => {
    let eventHandle: UnlistenFn | undefined;
    try {
      const msg = get(updatingMsg);
      loading.show({
        message: msg,
      });
      await tooltip(msg);

      listen<number>('compression_progress', (event) => {
        loading.show({
          message: `${msg}<br>压缩进度: ${event.payload}%`,
          html: true,
        });
        tooltip(`${msg}\n压缩进度: ${event.payload}%`).catch(console.error);
      }).then(
        (handle) => (eventHandle = handle),
        (error) => console.error(`Failed to listen to compression_progress: ${error}`),
      );

      await update(get(editData));

      notifySuccess(`${isEditMode.value ? '更新成功' : '创建失败'}`, undefined, 1000);
      return true;
    } catch (e) {
      console.error(e);
      notifyError(isEditMode.value ? '更新失败' : '创建失败', e, 1000);
      return false;
    } finally {
      loading.hide();
      eventHandle?.();
      await tooltip();
    }
  };

  const applyPreset = (preset: EditPreset) => {
    switch (preset) {
      case 'DoujinR18': {
        console.info('Applying preset: DoujinR18');
        updateField('content_info', {
          type: 'Game',
          data: {
            version: '1.0.0',
            game_type: 'RPG',
            developer: null,
            publisher: null,
            sys_platform: ['Windows'],
            distribution: {
              type: 'DLSite',
              data: {
                id: '',
                content_type: 'DoujinR18',
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

  return {
    isEditMode,
    editData,
    updateField,
    clearField,
    updateData,
    applyPreset,
  };
};
