import type {
  AppConfig,
  DeployArg,
  DLFetchArg,
  DLFetchInfo,
  Metadata,
  MetadataOption,
} from '@/api/types.ts';
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

  static async metadataDeploy(key: string, arg: DeployArg): Promise<void> {
    return await invoke('metadata_deploy', { key, arg });
  }

  static async metadataDeployOff(key: string): Promise<void> {
    return await invoke('metadata_deploy_off', { key });
  }

  static async metadataCollectionCache(): Promise<string[]> {
    return await invoke('metadata_collection_cache');
  }

  static async metadataDeploymentCache(): Promise<string[]> {
    return await invoke('metadata_deployment_cache');
  }

  static async metadataExport(): Promise<void> {
    return await invoke('metadata_export');
  }

  static async metadataImport(): Promise<void> {
    return await invoke('metadata_import');
  }

  static async utilProcessImgFile(source: string): Promise<string> {
    return await invoke('util_process_img_file', { source });
  }

  static async utilProcessImgWeb(url: string): Promise<string> {
    return await invoke('util_process_img_web', { url });
  }

  static async utilProcessImgBytes(data: [Uint8Array, number, number]): Promise<string> {
    return await invoke('util_process_img_bytes', { data });
  }

  static async utilClearUnusedImages(): Promise<number> {
    return await invoke('util_clear_unused_images');
  }

  static async utilClearUnusedDeployDirs(): Promise<number> {
    return await invoke('util_clear_unused_deploy_dirs');
  }

  static async utilRecalculateArchiveSize(): Promise<void> {
    return await invoke('util_recalculate_archive_size');
  }

  static async utilDlFetchInfo(arg: DLFetchArg): Promise<DLFetchInfo> {
    return await invoke('util_dl_fetch_info', { arg });
  }

  static async openConfigDir(): Promise<void> {
    return await invoke('open_config_dir');
  }

  static async openLogDir(): Promise<void> {
    return await invoke('open_log_dir');
  }

  static async pathResolveImg(hash: string): Promise<string> {
    return await invoke('path_resolve_img', { hash });
  }

  static async pathResolveArchive(path: string): Promise<string> {
    return await invoke('path_resolve_archive', { path });
  }

  static async configGet(): Promise<AppConfig> {
    return await invoke('config_get');
  }

  static async configUpdate(name: keyof AppConfig, value: unknown): Promise<void> {
    return await invoke('config_update', { name, value });
  }
}
