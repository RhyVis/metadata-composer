import type { Ref } from 'vue';
import type { DragDropPayload } from '@/api/event.ts';
import type { Metadata, MetadataOption } from '@/api/types.ts';
import type { LibraryStore } from '@/stores/library.ts';
import type { Event, UnlistenFn } from '@tauri-apps/api/event';
import { computed, inject, onMounted, onUnmounted, ref } from 'vue';
import { Command } from '@/api/cmd.ts';
import { LibraryKey } from '@/stores/library.ts';
import { listen, TauriEvent } from '@tauri-apps/api/event';
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
    tags: null,
    collection: null,
    image: null,
    content_info: null,
    archive_info: null,
    flag_create_archive: false,
  });

  const updateField = <K extends keyof MetadataOption>(field: K, value: MetadataOption[K]) => {
    editData.value[field] = value;
  };
  const clearField = <K extends keyof MetadataOption>(field: K) => {
    editData.value[field] = null as never;
  };
  const updateData = async () => await update(get(editData));

  let fileDropListener: UnlistenFn | null = null;

  onMounted(async () => {
    fileDropListener = await listen(TauriEvent.DRAG_DROP, async (event: Event<DragDropPayload>) => {
      for (const path of event.payload.paths) {
        if (path) {
          try {
            const hash = await Command.utilProcessImg(path);
            console.info(`Processed image with hash: ${hash}`);
            updateField('image', hash);
            break;
          } catch (e) {
            console.error('Error processing image:', e);
          }
        }
      }
    });
  });
  onUnmounted(() => {
    if (fileDropListener) {
      fileDropListener();
    }
  });

  return {
    isEditMode,
    originalData,
    editData,
    updateField,
    clearField,
    updateData,
  };
};
