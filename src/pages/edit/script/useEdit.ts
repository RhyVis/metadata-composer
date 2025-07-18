import type { Ref } from 'vue';
import type { DragDropPayload } from '@/api/event.ts';
import type { Metadata, MetadataOption } from '@/api/types.ts';
import type { Event, UnlistenFn } from '@tauri-apps/api/event';
import { useQuasar } from 'quasar';
import { computed, onMounted, onUnmounted, ref } from 'vue';
import { Command } from '@/api/cmd.ts';
import { extractFileName } from '@/api/util.ts';
import { useNotify } from '@/composables/useNotify.ts';
import { useLibraryStore } from '@/stores/library.ts';
import { listen, TauriEvent } from '@tauri-apps/api/event';
import { get } from '@vueuse/core';

export type UseEdit = ReturnType<typeof useEdit>;

type MaybeMetadata = Metadata | undefined;

export const useEdit = (initialData: Ref<MaybeMetadata>) => {
  const { update } = useLibraryStore();
  const { loading } = useQuasar();
  const { notifyWarning, notifySuccess } = useNotify();

  const isEditMode = computed(() => !!initialData.value?.id);

  const originalData = ref<MaybeMetadata>(get(initialData));
  const editData = ref<MetadataOption>({
    id: initialData.value?.id ?? null,
    title: initialData.value?.title ?? null,
    alias: initialData.value?.alias ?? null,
    tags: initialData.value?.tags ?? null,
    collection: initialData.value?.collection ?? null,
    description: initialData.value?.description ?? null,
    image: initialData.value?.image ?? null,
    content_info: initialData.value?.content_info ?? null,
    archive_info: initialData.value?.archive_info ?? null,
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

  onMounted(() => {
    listen(TauriEvent.DRAG_DROP, async (event: Event<DragDropPayload>) => {
      if (!event.payload.paths || event.payload.paths.length === 0) {
        console.warn('No valid file paths provided in drag-drop event');
        return;
      }
      loading.show({
        message: '正在处理拖放的文件...',
      });
      for (const path of event.payload.paths) {
        if (path) {
          console.info(`Dragged and dropped file: '${path}'`);
          const valid =
            path.endsWith('png') ||
            path.endsWith('jpg') ||
            path.endsWith('jpeg') ||
            path.endsWith('webp');
          const fileName = extractFileName(path);
          if (!valid) {
            console.warn(`Invalid file type: '${fileName}'`);
            notifyWarning(
              `不支持的图片文件类型: ${fileName}`,
              '请拖放 PNG、JPG、JPEG 或 WEBP 格式的图片文件。',
            );
            continue;
          }
          try {
            const hash = await Command.utilProcessImg(path);
            console.info(`Processed image with hash: ${hash}`);
            updateField('image', hash);
            notifySuccess('图片处理成功', `${fileName} -> ${hash}`);
            break;
          } catch (e) {
            console.error(`Error processing image: ${e}`);
          }
        }
      }
      loading.hide();
    }).then(
      (handle) => (fileDropListener = handle),
      (error) => console.error('Failed to listen for drag-drop events:', error),
    );
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
