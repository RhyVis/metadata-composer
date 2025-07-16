import { open } from '@tauri-apps/plugin-dialog';

export async function selectFile(): Promise<string> {
  const file = await open({
    multiple: false,
    directory: false,
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
