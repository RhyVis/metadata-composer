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
