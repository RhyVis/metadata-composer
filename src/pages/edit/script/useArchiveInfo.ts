import type { ArchiveInfo } from '@/api/types.ts';
import type { UseEdit } from '@/pages/edit/script/useEdit.ts';
import { computed } from 'vue';
import { selectDirectory, selectFile } from '@/api/dialog.ts';
import { useNotify } from '@/composables/useNotify.ts';
import { set } from '@vueuse/core';

const defaultArchiveInfo = (type: ArchiveInfo['type'] = 'None'): ArchiveInfo => {
  switch (type) {
    case 'None': {
      return {
        type: 'None',
      };
    }
    case 'ArchiveFile': {
      return {
        type: 'ArchiveFile',
        data: {
          size: 0,
          path: '',
          password: null,
        },
      };
    }
    case 'CommonFile': {
      return {
        type: 'CommonFile',
        data: {
          size: 0,
          path: '',
        },
      };
    }
    case 'Directory': {
      return {
        type: 'Directory',
        data: {
          size: 0,
          path: '',
        },
      };
    }
  }
};

export const defaultPassword = 'COMPOSER';

export const useArchiveInfo = (edit: UseEdit) => {
  const { editData, updateField } = edit;
  const { notifyError, notifyWarning } = useNotify();

  if (!editData.value.archive_info) {
    console.info('Initializing archive_info with default values');
    editData.value.archive_info = defaultArchiveInfo();
  }

  const archiveInfo = computed<ArchiveInfo>(() => editData.value.archive_info!);
  const inputPath = computed({
    get: () => (archiveInfo.value.type === 'None' ? '' : archiveInfo.value.data.path),
    set: (val: string) => {
      if (archiveInfo.value.type === 'None') {
        console.warn('Attempted to set path on non-archive type');
      } else {
        updateField('archive_info', {
          type: archiveInfo.value.type,
          data: {
            ...archiveInfo.value.data,
            path: val ?? '',
          },
        } as ArchiveInfo);
      }
    },
  });
  const inputPassword = computed({
    get: () => (archiveInfo.value.type === 'ArchiveFile' ? archiveInfo.value.data.password : ''),
    set: (val: string | null) => {
      if (archiveInfo.value.type !== 'ArchiveFile') {
        console.warn('Attempted to set password on non-archive type');
      } else {
        updateField('archive_info', {
          type: 'ArchiveFile',
          data: {
            ...archiveInfo.value.data,
            password: val?.trim() || null,
          },
        });
      }
    },
  });

  const archiveType = computed({
    get: () => archiveInfo.value.type,
    set: (val: ArchiveInfo['type']) => updateField('archive_info', defaultArchiveInfo(val)),
  });

  const doSelect = async (dir: boolean) => {
    if (archiveType.value === 'None') {
      console.warn('Attempted to select path for non-archive type');
      return;
    }
    try {
      const path = dir ? await selectDirectory() : await selectFile();
      if (!path) {
        notifyWarning('未选择路径', '请确保选择了一个有效的文件或目录路径');
        return;
      }

      if (archiveType.value === 'ArchiveFile' && !inputPassword.value) {
        set(inputPassword, defaultPassword);
      }

      const obj = { ...archiveInfo.value } as Exclude<ArchiveInfo, { type: 'None' }>;
      obj.data.path = path;
      updateField('archive_info', obj);
    } catch (e) {
      console.error(e);
      notifyError('选择路径失败', e);
    }
  };

  const flagCreateArchive = computed({
    get: () => editData.value.flag_create_archive,
    set: (val: boolean) => updateField('flag_create_archive', val),
  });

  return { archiveType, inputPath, inputPassword, flagCreateArchive, doSelect };
};
