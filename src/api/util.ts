import { Command } from '@/api/cmd.ts';
import { convertFileSrc } from '@tauri-apps/api/core';

export function formatBytes(bytes: number): string {
  if (bytes === 0) return '0B';
  const units = ['B', 'KB', 'MB', 'GB', 'TB'];
  const k = 1024;
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  const size = bytes / Math.pow(k, i);
  return `${size.toFixed(2)}${units[i]}`;
}

export async function resolveImageAsset(hash: string | null | undefined): Promise<string> {
  if (!hash) return '';
  return convertFileSrc(await Command.pathResolveImg(hash));
}

export const extractFileName = (path: string): string => {
  if (!path) return '';
  const normalizedPath = path.replace(/\\/g, '/');
  const parts = normalizedPath.split('/');
  return parts.filter(Boolean).pop() || '';
};

export function generateRandomAsciiString(length: number = 16): string {
  const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
  let result = '';
  for (let i = 0; i < length; i++) {
    const randomIndex = Math.floor(Math.random() * characters.length);
    result += characters[randomIndex];
  }
  return result;
}

export function isNumericOnly(str: string): boolean {
  return /^\d+$/.test(str);
}

export function truncateString(str: string, maxLength: number = 40): string {
  if (str.length <= maxLength) return str;
  return str.slice(0, maxLength - 3) + '...';
}

/**
 * Used to validate a string for use as a file or directory name in the file system.
 *
 * @return true if valid, or a string describing the validation error.
 */
export function isValidFileSystemString(str: string): true | string {
  // oxlint-disable-next-line no-control-regex
  const invalidChars = /[<>:"/\\|?*\x00-\x1F]/;
  if (invalidChars.test(str)) {
    return '不能包含以下字符: < > : " / \\ | ? *';
  }

  const reservedNames = /^(CON|PRN|AUX|NUL|COM[1-9]|LPT[1-9])$/i;
  if (reservedNames.test(str) || reservedNames.test(str.split('.')[0])) {
    return `"${str}" 是系统保留名称`;
  }

  if (str.startsWith(' ') || str.endsWith(' ') || str.endsWith('.')) {
    return '不能以空格开头或结尾';
  }

  if (str.endsWith('.')) {
    return '不能以点号结尾';
  }

  if (str.length > 255) {
    return '长度不能超过 255 个字符';
  }

  return true;
}

export function isAsciiNumeric(str: string): boolean {
  return /^[\x20-\x7E]+$/.test(str);
}
