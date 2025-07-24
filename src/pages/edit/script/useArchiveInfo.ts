import type { ArchiveInfo } from '@/api/types.ts';
import type { UseEdit } from '@/pages/edit/script/useEdit.ts';
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import { selectDirectory, selectFile } from '@/api/dialog.ts';
import { useNotify } from '@/hooks/useNotify';
import { ArchiveTypeEnum } from '@/pages/edit/script/define';
import { set } from '@vueuse/core';

const defaultArchiveInfo = (type: ArchiveInfo['type'] = ArchiveTypeEnum.None): ArchiveInfo => {
  switch (type) {
    case ArchiveTypeEnum.ArchiveFile: {
      return {
        type: ArchiveTypeEnum.ArchiveFile,
        data: {
          size: 0,
          path: '',
          password: null,
        },
      };
    }
    case ArchiveTypeEnum.CommonFile: {
      return {
        type: ArchiveTypeEnum.CommonFile,
        data: {
          size: 0,
          path: '',
        },
      };
    }
    case ArchiveTypeEnum.Directory: {
      return {
        type: ArchiveTypeEnum.Directory,
        data: {
          size: 0,
          path: '',
        },
      };
    }
    default: {
      return {
        type: ArchiveTypeEnum.None,
      };
    }
  }
};

export const defaultPassword = 'COMPOSER';

export const useArchiveInfo = (edit: UseEdit) => {
  const { t } = useI18n();
  const { editData, updateField } = edit;
  const { notifyError, notifyWarning } = useNotify();

  if (!editData.value.archive_info) {
    console.info('Initializing archive_info with default values');
    editData.value.archive_info = defaultArchiveInfo();
  }

  const archiveInfo = computed<ArchiveInfo>(() => editData.value.archive_info!);
  const inputPath = computed({
    get: () => (archiveInfo.value.type === ArchiveTypeEnum.None ? '' : archiveInfo.value.data.path),
    set: (val: string) => {
      if (archiveInfo.value.type === ArchiveTypeEnum.None) {
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
    if (archiveType.value === ArchiveTypeEnum.None) {
      console.warn('Attempted to select path for non-archive type');
      return;
    }
    try {
      const path = dir ? await selectDirectory() : await selectFile();
      if (!path) {
        notifyWarning(t('notify.select-path.no-path'), t('notify.select-path.valid-please'));
        return;
      }

      if (archiveType.value === ArchiveTypeEnum.ArchiveFile && !inputPassword.value) {
        set(inputPassword, defaultPassword);
      }

      const obj = { ...archiveInfo.value } as Exclude<ArchiveInfo, { type: 'None' }>;
      obj.data.path = path;
      updateField('archive_info', obj);
    } catch (e) {
      console.error(e);
      notifyError(t('notify.select-path.fail'), e);
    }
  };

  const flagCreateArchive = computed({
    get: () => editData.value.flag_create_archive,
    set: (val: boolean) => updateField('flag_create_archive', val),
  });

  return { archiveType, inputPath, inputPassword, flagCreateArchive, doSelect };
};
