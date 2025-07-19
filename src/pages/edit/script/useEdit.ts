import type { Ref } from 'vue';
import type { Metadata, MetadataOption } from '@/api/types.ts';
import type { EditPreset } from '@/pages/edit/script/define.ts';
import { cloneDeep } from 'lodash-es';
import { computed, ref } from 'vue';
import { useLibraryStore } from '@/stores/library.ts';
import { get } from '@vueuse/core';

type MaybeMetadata = Metadata | undefined;

export type UseEdit = ReturnType<typeof useEdit>;

export const useEdit = (initialData: Ref<MaybeMetadata>) => {
  const { update } = useLibraryStore();

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

  const updateData = async () => await update(get(editData));

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
