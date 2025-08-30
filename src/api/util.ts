import { Command } from '@/api/cmd.ts';
import i18n from '@/i18n';
import { convertFileSrc } from '@tauri-apps/api/core';

const { t } = i18n.global;

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

export function extractNumbers(str: string): string {
  return str ? str.replace(/\D/g, '') : '';
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
    return `${t('general.invalid-char')}: < > : " / \\ | ? *`;
  }

  const reservedNames = /^(CON|PRN|AUX|NUL|COM[1-9]|LPT[1-9])$/i;
  if (reservedNames.test(str) || reservedNames.test(str.split('.')[0])) {
    return t('general.system-preserve-name', [str]);
  }

  if (str.startsWith(' ') || str.endsWith(' ')) {
    return t('general.invalid-space');
  }

  if (str.endsWith('.')) {
    return t('general.invalid-dot');
  }

  if (str.length > 255) {
    return t('general.invalid-length');
  }

  return true;
}

export function isAsciiNumeric(str: string): boolean {
  return /^[\x20-\x7E]+$/.test(str);
}
