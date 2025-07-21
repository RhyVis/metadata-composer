import type { DragDropPayload } from '@/api/event.ts';
import type { UseEdit } from '@/pages/edit/script/useEdit.ts';
import type { Event, UnlistenFn } from '@tauri-apps/api/event';
import { useQuasar } from 'quasar';
import { onMounted, onUnmounted, ref, watch } from 'vue';
import { Command } from '@/api/cmd.ts';
import { extractFileName, truncateString } from '@/api/util.ts';
import { useNotify } from '@/composables/useNotify.ts';
import { convertFileSrc } from '@tauri-apps/api/core';
import { listen, TauriEvent } from '@tauri-apps/api/event';
import { readImage, readText } from '@tauri-apps/plugin-clipboard-manager';
import { set, useToggle } from '@vueuse/core';

export const useImg = (edit: UseEdit) => {
  const { editData, updateField } = edit;
  const { loading } = useQuasar();
  const { notifyWarning, notifySuccess, notifyError } = useNotify();

  const [showImage, setShowImage] = useToggle(false);

  const pasteImg = async () => {
    try {
      try {
        const image = await readImage();
        const size = await image.size();
        loading.show({
          message: `正在处理粘贴的图像... (${size.width}x${size.height})`,
        });
        const hash = await Command.utilProcessImgBytes([
          await image.rgba(),
          size.width,
          size.height,
        ]);
        console.info(`Processed pasted image with hash: ${hash}`);
        updateField('image', hash);
        notifySuccess('图片处理成功', `粘贴图像 -> ${hash}`);
        return;
      } catch (e) {
        if (
          e ==
          'The clipboard contents were not available in the requested format or the clipboard is empty.'
        ) {
          console.info('Maybe not pasting image, trying text instead');
        } else {
          console.error('Not able to read image from clipboard');
        }
      } finally {
        loading.hide();
      }

      await requestImage(await readText());
    } catch (e) {
      console.error(`Failed to paste image link: ${e}`);
    }
  };
  const requestImage = async (url: string) => {
    const trim = url.trim();
    if (!trim) {
      notifyWarning('请使用有效链接');
      return;
    }

    console.info(`Processing image from URL: '${trim}'`);

    try {
      loading.show({
        message: `正在处理图像链接: ${truncateString(trim)}`,
      });
      const hash = await Command.utilProcessImgWeb(trim);
      console.info(`Processed image with hash: ${hash}`);
      updateField('image', hash);
      notifySuccess('图片处理成功', `${truncateString(trim)} -> ${hash}`);
    } catch (e) {
      console.error(e);
      notifyError(`处理 '${truncateString(trim)}' 图像失败`, e);
    } finally {
      loading.hide();
    }
  };

  const clearImage = () => {
    updateField('image', null);
    setShowImage(false);
  };

  const imageSrc = ref<string | undefined>(undefined);
  const updateAssetUrl = async (hash: string | null) => {
    if (hash) {
      try {
        const path = await Command.pathResolveImg(hash);
        const assetUrl = convertFileSrc(path);
        set(imageSrc, assetUrl);
      } catch (e) {
        console.error(e);
      }
    } else {
      set(imageSrc, undefined);
    }
  };

  watch(
    () => editData.value.image,
    async (newHash) => await updateAssetUrl(newHash),
  );

  let fileDropListener: UnlistenFn | null = null;

  onMounted(() => {
    updateAssetUrl(editData.value.image).catch(console.error);
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
            const hash = await Command.utilProcessImgFile(path);
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
  onUnmounted(() => fileDropListener?.());

  return { imageSrc, showImage, setShowImage, pasteImg, clearImage };
};
