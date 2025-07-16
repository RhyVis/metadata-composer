import type { Metadata, MetadataOption } from '@/api/types.ts';
import { invoke } from '@tauri-apps/api/core';

export class Command {
  static async libUpdate(opt: MetadataOption): Promise<string | null> {
    return await invoke('lib_update', { opt });
  }

  static async libGetAll(): Promise<Metadata[]> {
    return await invoke('lib_get_all');
  }

  static async libGet(key: string): Promise<Metadata | null> {
    return await invoke('lib_get', { key });
  }
}
