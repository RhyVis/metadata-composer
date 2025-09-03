import type { DialogFilter } from '@tauri-apps/plugin-dialog';
import i18n from '@/i18n';
import { open } from '@tauri-apps/plugin-dialog';

const { t } = i18n.global;

export async function selectFile(filters?: DialogFilter[]): Promise<string> {
  const file = await open({
    multiple: false,
    directory: false,
    filters: filters || [],
  });

  if (!file) {
    throw new Error(t('general.file-none'));
  }

  return file;
}

export async function selectDirectory(): Promise<string> {
  const dir = await open({
    multiple: false,
    directory: true,
  });

  if (!dir) {
    throw new Error(t('general.folder-none'));
  }

  return dir;
}
