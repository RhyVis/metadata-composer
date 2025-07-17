import type { Metadata, MetadataOption } from '@/api/types.ts';
import { invoke } from '@tauri-apps/api/core';

export class Command {
  static async metadataUpdate(opt: MetadataOption): Promise<string | null> {
    return await invoke('metadata_update', { opt });
  }

  static async metadataGetAll(): Promise<Metadata[]> {
    return await invoke('metadata_get_all');
  }

  static async metadataGet(key: string): Promise<Metadata | null> {
    return await invoke('metadata_get', { key });
  }

  static async metadataDelete(key: string): Promise<void> {
    await invoke('metadata_delete', { key });
  }

  static async metadataCollectionList(): Promise<string[]> {
    return await invoke('metadata_collection_list');
  }
}
