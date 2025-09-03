import type { ArchiveInfo, Metadata } from '@/api/types.ts';
import i18n from '@/i18n';

const { t } = i18n.global;

export function joinArray(arr: string[], sep = ', '): string {
  return arr.length > 0 ? arr.join(sep) : t('page.main.table.unknown');
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

export function getDeployPath(data: Metadata): string {
  return data.deploy_info.type === 'Directory' ? data.deploy_info.data.path : '';
}
