import type { ArchiveInfo } from '@/api/types.ts';
import type { UseEdit } from '@/pages/edit/script/useEdit.ts';
import { computed, ref, watch } from 'vue';
import { selectDirectory, selectFile } from '@/api/dialog.ts';
import { ArchiveTypeEnum } from '@/pages/edit/script/define.ts';
import { get, set } from '@vueuse/core';

export const useArchiveInfo = (edit: UseEdit) => {
  const { editData, originalData, updateField } = edit;

  const inputPath = ref<string | null>(null);
  const inputPassword = ref<string | null>(null);

  const getInitialType = (): string => {
    const data: ArchiveInfo | null = editData.value.archive_info
      ? editData.value.archive_info
      : originalData.value?.archive_info
        ? originalData.value.archive_info
        : null;
    if (!data) return ArchiveTypeEnum.None;

    function extractType(info: ArchiveInfo): string {
      if (info === 'None') return ArchiveTypeEnum.None;
      if ('ArchiveFile' in info) {
        set(inputPath, info.ArchiveFile.path);
        set(inputPassword, info.ArchiveFile.password);
        return ArchiveTypeEnum.ArchiveFile;
      }
      if ('CommonFile' in info) {
        set(inputPath, info.CommonFile.path);
        return ArchiveTypeEnum.CommonFile;
      }
      if ('Directory' in info) {
        set(inputPath, info.Directory.path);
        return ArchiveTypeEnum.Directory;
      }
      return ArchiveTypeEnum.None;
    }

    return extractType(data);
  };

  const generateArchiveInfo = (): ArchiveInfo => {
    switch (currentType.value) {
      case ArchiveTypeEnum.None: {
        return 'None';
      }
      case ArchiveTypeEnum.ArchiveFile: {
        const pwd = get(inputPassword);
        return pwd
          ? {
              ArchiveFile: {
                size: 0,
                path: get(inputPath) ?? '',
                password: get(inputPassword),
              },
            }
          : {
              ArchiveFile: {
                size: 0,
                path: get(inputPath) ?? '',
                password: null,
              },
            };
      }
      case ArchiveTypeEnum.CommonFile: {
        return {
          CommonFile: {
            size: 0,
            path: get(inputPath) ?? '',
          },
        };
      }
      case ArchiveTypeEnum.Directory: {
        return {
          Directory: {
            size: 0,
            path: get(inputPath) ?? '',
          },
        };
      }
      default: {
        console.warn('Unexpected archive type:', currentType.value);
        set(currentType, ArchiveTypeEnum.None);
        return 'None';
      }
    }
  };

  const doSelect = async (dir: boolean) => {
    try {
      set(inputPath, dir ? await selectDirectory() : await selectFile());
    } catch (e) {
      console.error(e);
    }
  };

  const currentType = ref(getInitialType());

  const flagCreateArchive = computed({
    get: () => editData.value.flag_create_archive,
    set: (value: boolean) => {
      console.info(`Setting flag_create_archive to: ${value}`);
      updateField('flag_create_archive', value);
    },
  });

  watch(currentType, (newType) => {
    console.info(`Archive type changed: ${newType}`);
    if (newType == ArchiveTypeEnum.None) {
      updateField('archive_info', newType);
    } else {
      set(inputPath, null);
      set(inputPassword, null);
      updateField('archive_info', generateArchiveInfo());
    }
  });

  watch(inputPath, (newPath) => {
    if (!newPath) return;
    updateField('archive_info', generateArchiveInfo());
  });
  watch(inputPassword, (newPassword) => {
    if (!newPassword) return;
    updateField('archive_info', generateArchiveInfo());
  });

  return { currentType, inputPath, inputPassword, flagCreateArchive, doSelect };
};
