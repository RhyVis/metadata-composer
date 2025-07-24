import type { DragDropPayload } from '@/api/event.ts';
import type { UseEdit } from '@/pages/edit/script/useEdit.ts';
import type { Event, UnlistenFn } from '@tauri-apps/api/event';
import { useQuasar } from 'quasar';
import { onMounted, onUnmounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { Command } from '@/api/cmd.ts';
import { extractFileName, truncateString } from '@/api/util.ts';
import { useNotify } from '@/hooks/useNotify';
import { convertFileSrc } from '@tauri-apps/api/core';
import { listen, TauriEvent } from '@tauri-apps/api/event';
import { readImage, readText } from '@tauri-apps/plugin-clipboard-manager';
import { set, useToggle } from '@vueuse/core';

export const useImg = (edit: UseEdit) => {
  const { t } = useI18n();
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
          message: t('page.edit.image.loading.paste-img-process', [size.width, size.height]),
        });
        const hash = await Command.utilProcessImgBytes([
          await image.rgba(),
          size.width,
          size.height,
        ]);
        console.info(`Processed pasted image with hash: ${hash}`);
        updateField('image', hash);
        notifySuccess(
          t('page.edit.image.notify.success'),
          t('page.edit.image.notify.paste-hash-display', [hash]),
        );
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
      notifyWarning(t('page.edit.image.notify.valid-url-please'));
      return;
    }

    console.info(`Processing image from URL: '${trim}'`);

    try {
      loading.show({
        message: t('page.edit.image.loading.paste-url-process', [truncateString(trim)]),
      });
      const hash = await Command.utilProcessImgWeb(trim);
      console.info(`Processed image with hash: ${hash}`);
      updateField('image', hash);
      notifySuccess(t('page.edit.image.notify.success'), `${truncateString(trim)} -> ${hash}`);
    } catch (e) {
      console.error(e);
      notifyError(t('page.edit.image.notify.url-fail', [truncateString(trim)]), e);
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
        message: t('page.edit.image.loading.paste-drag-process'),
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
              t('page.edit.image.notify.unsupported-type', [fileName]),
              t('page.edit.image.notify.unsupported-hint'),
            );
            continue;
          }
          try {
            const hash = await Command.utilProcessImgFile(path);
            console.info(`Processed image with hash: ${hash}`);
            updateField('image', hash);
            notifySuccess(t('page.edit.image.notify.success'), `${fileName} -> ${hash}`);
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
