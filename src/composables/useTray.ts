import { Tray } from '@/api/tray.ts';

async function tooltip(text: string | null = null): Promise<void> {
  try {
    await Tray.setTooltip(text);
  } catch (e) {
    console.error(`Failed to set tray tooltip: ${text}`, e);
  }
}

export const useTray = () => {
  return { tooltip };
};
