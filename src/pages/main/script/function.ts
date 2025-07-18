import type { ArchiveInfo, ContentInfo, Metadata } from '@/api/types.ts';
import { ArchiveTypeEnum } from '@/pages/edit/script/define.ts';

export function joinArray(arr: string[], sep = ', '): string {
  return arr.length > 0 ? arr.join(sep) : '<未提供>';
}

export function extractContentType(contentInfo: ContentInfo): string {
  if (typeof contentInfo === 'string') {
    return contentInfo;
  } else if ('Game' in contentInfo) {
    return '游戏';
  } else {
    return '<未提供>';
  }
}

export function extractArchiveSize(archiveInfo: ArchiveInfo): number {
  if (typeof archiveInfo === 'string') {
    return 0;
  } else if (ArchiveTypeEnum.ArchiveFile in archiveInfo) {
    return archiveInfo.ArchiveFile.size || 0;
  } else if (ArchiveTypeEnum.CommonFile in archiveInfo) {
    return archiveInfo.CommonFile.size || 0;
  } else if (ArchiveTypeEnum.Directory in archiveInfo) {
    return archiveInfo.Directory.size || 0;
  } else {
    return 0;
  }
}

export function isDeployable(data: Metadata): boolean {
  return data.archive_info != 'None' && data.deploy_info == 'None';
}

export function isDeployed(data: Metadata): boolean {
  return data.deploy_info != 'None';
}
