import type { QForm } from 'quasar';
import type { Ref } from 'vue';
import type { CompressionInfoPayload } from '@/api/event.ts';
import type { Metadata, MetadataOption } from '@/api/types.ts';
import type { EditPreset } from '@/pages/edit/script/define.ts';
import type { UnlistenFn } from '@tauri-apps/api/event';
import { cloneDeep } from 'lodash-es';
import { useQuasar } from 'quasar';
import { computed, ref } from 'vue';
import { truncateString } from '@/api/util.ts';
import { useNotify } from '@/hooks/useNotify';
import { useTray } from '@/hooks/useTray';
import { useLibraryStore } from '@/stores/library.ts';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { sendNotification } from '@tauri-apps/plugin-notification';
import { get, useToggle } from '@vueuse/core';

export type UseEdit = ReturnType<typeof useEdit>;

export type MaybeMetadata = Metadata | undefined;

type EditableField = Exclude<keyof MetadataOption, 'id'>;

const window = getCurrentWindow();

export const useEdit = (id: Ref<string>, formRef: Ref<QForm>) => {
  const { update, index } = useLibraryStore();
  const { loading } = useQuasar();
  const { notifySuccess, notifyError, notifyWarning } = useNotify();
  const { tooltip } = useTray();

  const initData = () => index(get(id));
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

  const updatingMsg = computed(
    () =>
      `正在${isEditMode.value ? '更新 ' : '创建 '}${editData.value.title || editData.value.id || '未知'} 数据...`,
  );

  const updateData = async (): Promise<boolean> => {
    if (!(await validate())) {
      notifyWarning('表单验证失败，请检查填写内容');
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
          message: `${msg}<br>压缩进度：${progress}%<br>文件数量：${fileCount}<br>当前文件：${currentFile}`,
          html: true,
        });
        tooltip(
          `${msg}\n压缩进度：${progress}%\n文件数量：${fileCount}\n当前文件：${currentFile}`,
        ).catch(console.error);
      }).then(
        (handle) => (eventHandle = handle),
        (error) => console.error(`Failed to listen to compression_progress: ${error}`),
      );

      setPageLock(true);
      await update(get(editData));
      setEverEdited(false);

      const successMsg = isEditMode.value ? '更新成功' : '创建成功';
      notifySuccess(successMsg, undefined, 1000);
      if (!(await window.isVisible())) sendNotification(successMsg);

      return true;
    } catch (e) {
      console.error(e);
      notifyError(isEditMode.value ? '更新失败' : '创建失败', e, 1000);
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

  const validate = async () => {
    try {
      return await formRef.value.validate();
    } catch (e) {
      console.error(e);
      notifyError('表单验证失败', e);
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
