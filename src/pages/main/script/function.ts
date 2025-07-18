import type { ArchiveInfo, ContentInfo } from '@/api/types.ts';

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
  } else if ('size' in archiveInfo) {
    return archiveInfo.size as number;
  } else {
    return 0;
  }
}
