import type { Ref } from 'vue';
import type { Metadata, MetadataOption } from '@/api/types.ts';
import type { LibraryStore } from '@/stores/library.ts';
import { computed, inject, ref } from 'vue';
import { LibraryKey } from '@/stores/library.ts';
import { get } from '@vueuse/core';

export type UseEdit = ReturnType<typeof useEdit>;

export const useEdit = (initialData: Ref<Metadata | undefined>) => {
  const { update }: LibraryStore = inject(LibraryKey)!;

  const isEditMode = computed(() => !!initialData.value?.id);

  const originalData = ref<Metadata | undefined>(get(initialData));
  const editData = ref<MetadataOption>({
    id: initialData.value?.id ?? null,
    title: null,
    alias: null,
    archive_info: null,
    collection: null,
    content_type: null,
    tags: null,
  });

  const updateField = <K extends keyof MetadataOption>(field: K, value: MetadataOption[K]) => {
    editData.value[field] = value;
  };
  const clearField = <K extends keyof MetadataOption>(field: K) => {
    editData.value[field] = null;
  };
  const updateData = async () => {
    await update(get(editData));
  };

  return {
    isEditMode,
    originalData,
    editData,
    updateField,
    clearField,
    updateData,
  };
};
