import type { DeployArg, InternalConfig, Metadata, MetadataOption } from '@/api/types.ts';
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

  static async metadataDeploy(key: string, arg: DeployArg): Promise<void> {
    return await invoke('metadata_deploy', { key, arg });
  }

  static async metadataExport(): Promise<void> {
    return await invoke('metadata_export');
  }

  static async metadataImport(): Promise<void> {
    return await invoke('metadata_import');
  }

  static async metadataDeployOff(key: string): Promise<void> {
    return await invoke('metadata_deploy_off', { key });
  }

  static async utilProcessImg(source: string): Promise<string> {
    return await invoke('util_process_img', { source });
  }

  static async utilClearUnusedImages(): Promise<number> {
    return await invoke('util_clear_unused_images');
  }

  static async pathResolveImg(hash: string): Promise<string> {
    return await invoke('path_resolve_img', { hash });
  }

  static async configGet(): Promise<InternalConfig> {
    return await invoke('config_get');
  }

  static async configUpdate(name: keyof InternalConfig, value: unknown): Promise<void> {
    return await invoke('config_update', { name, value });
  }
}
