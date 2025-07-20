import type { Ref } from 'vue';
import type { Metadata, MetadataOption } from '@/api/types.ts';
import type { EditPreset } from '@/pages/edit/script/define.ts';
import { cloneDeep } from 'lodash-es';
import { useQuasar } from 'quasar';
import { computed, ref } from 'vue';
import { useNotify } from '@/composables/useNotify.ts';
import { useTray } from '@/composables/useTray.ts';
import { useLibraryStore } from '@/stores/library.ts';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { get } from '@vueuse/core';

export type UseEdit = ReturnType<typeof useEdit>;

export type MaybeMetadata = Metadata | undefined;

const window = getCurrentWindow();

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

  const updateData = async (): Promise<boolean> => {
    try {
      const msg = `正在${isEditMode.value ? '更新 ' : '保存 '}${editData.value.title || editData.value.id} 数据...`;
      loading.show({
        message: msg,
      });
      await tooltip(msg);
      const hideWindow = setTimeout(async () => {
        await window.hide();
      }, 2442);

      await update(get(editData));

      clearTimeout(hideWindow);
      notifySuccess('保存成功', undefined, 1000);
      return true;
    } catch (e) {
      console.error(e);
      notifyError(isEditMode.value ? '更新失败' : '保存失败', e, 1000);
      return false;
    } finally {
      loading.hide();
      await tooltip();
      await window.show();
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
            developer: null,
            publisher: null,
            sys_platform: [],
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
