import type { ArchiveInfo } from '@/api/types.ts';
import type { UseEdit } from '@/pages/edit/script/useEdit.ts';
import { computed, ref, watch } from 'vue';
import { selectDirectory, selectFile } from '@/api/dialog.ts';
import { ArchiveTypeEnum } from '@/pages/edit/script/define.ts';
import { get, set } from '@vueuse/core';

export const useArchiveInfo = (edit: UseEdit) => {
  const { editData, originalData, updateField } = edit;

  const getInitialType = (): string => {
    if (!editData.value.archive_info) {
      if (!originalData.value?.archive_info) {
        return ArchiveTypeEnum.None;
      }

      if (originalData.value.archive_info == ArchiveTypeEnum.None) {
        return ArchiveTypeEnum.None;
      } else if (ArchiveTypeEnum.ArchiveFile in originalData.value.archive_info) {
        return ArchiveTypeEnum.ArchiveFile;
      } else if (ArchiveTypeEnum.CommonFile in originalData.value.archive_info) {
        return ArchiveTypeEnum.CommonFile;
      } else if (ArchiveTypeEnum.Directory in originalData.value.archive_info) {
        return ArchiveTypeEnum.Directory;
      }

      return ArchiveTypeEnum.None;
    }

    if (editData.value.archive_info == ArchiveTypeEnum.None) {
      return ArchiveTypeEnum.None;
    } else if (ArchiveTypeEnum.ArchiveFile in editData.value.archive_info) {
      return ArchiveTypeEnum.ArchiveFile;
    } else if (ArchiveTypeEnum.CommonFile in editData.value.archive_info) {
      return ArchiveTypeEnum.CommonFile;
    } else if (ArchiveTypeEnum.Directory in editData.value.archive_info) {
      return ArchiveTypeEnum.Directory;
    }

    return ArchiveTypeEnum.None;
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

  const inputPath = ref<string | null>(null);
  const inputPassword = ref<string | null>(null);

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

  return { currentType, inputPath, inputPassword, doSelect };
};
