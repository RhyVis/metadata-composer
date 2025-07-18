import type { DialogFilter } from '@tauri-apps/plugin-dialog';
import { open } from '@tauri-apps/plugin-dialog';

export async function selectFile(filters?: DialogFilter[]): Promise<string> {
  const file = await open({
    multiple: false,
    directory: false,
    filters: filters || [],
  });

  if (!file) {
    throw new Error('未选择文件');
  }

  return file;
}

export async function selectDirectory(): Promise<string> {
  const dir = await open({
    multiple: false,
    directory: true,
  });

  if (!dir) {
    throw new Error('未选择目录');
  }

  return dir;
}
