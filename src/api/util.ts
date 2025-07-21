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

export function isValidFileSystemString(str: string): true | string {
  // Windows/Unix 系统中的保留字符
  // oxlint-disable-next-line no-control-regex
  const invalidChars = /[<>:"/\\|?*\x00-\x1F]/;
  if (invalidChars.test(str)) {
    return '文件名不能包含以下字符: < > : " / \\ | ? *';
  }

  // Windows 系统的保留文件名
  const reservedNames = /^(CON|PRN|AUX|NUL|COM[1-9]|LPT[1-9])$/i;
  if (reservedNames.test(str) || reservedNames.test(str.split('.')[0])) {
    return `"${str}" 是系统保留名称`;
  }

  // 检查文件名开头或结尾的空格和点号
  if (str.startsWith(' ') || str.endsWith(' ') || str.endsWith('.')) {
    return '文件名不能以空格开头或结尾，也不能以点号结尾';
  }

  // 检查文件名长度（一般 Windows 限制为 255 个字符）
  if (str.length > 255) {
    return '文件名长度不能超过 255 个字符';
  }

  return true;
}
