import type { ArchiveInfo, Metadata } from '@/api/types.ts';

export function joinArray(arr: string[], sep = ', '): string {
  return arr.length > 0 ? arr.join(sep) : '<未提供>';
}

export function extractArchiveSize(archiveInfo: ArchiveInfo): number {
  switch (archiveInfo.type) {
    case 'None':
      return 0;
    case 'ArchiveFile':
    case 'CommonFile':
    case 'Directory':
      return archiveInfo.data.size;
    default:
      console.warn(`Unknown archive type: ${JSON.stringify(archiveInfo)}`);
      return 0;
  }
}

export function isDeployable(data: Metadata): boolean {
  return data.archive_info.type !== 'None' && data.deploy_info.type === 'None';
}

export function isDeployed(data: Metadata): boolean {
  return data.deploy_info.type !== 'None';
}
