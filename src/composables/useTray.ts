import type { Image } from '@tauri-apps/api/image';
import type { TrayIconOptions } from '@tauri-apps/api/tray';
import { defaultWindowIcon } from '@tauri-apps/api/app';
import { Menu } from '@tauri-apps/api/menu/menu';
import { TrayIcon } from '@tauri-apps/api/tray';
import { getCurrentWindow } from '@tauri-apps/api/window';

const window = getCurrentWindow();

let tray: TrayIcon | null = null;

async function init(): Promise<TrayIcon> {
  const menu = await Menu.new({
    items: [
      {
        id: 'quit',
        text: '退出',
        action: () => window.close().catch(console.error),
      },
    ],
  });

  const options: TrayIconOptions = {
    menu: menu,
    icon: (await defaultWindowIcon()) as Image,
    showMenuOnLeftClick: false,
    action(event) {
      switch (event.type) {
        case 'DoubleClick': {
          console.info('Tray icon double-clicked');
          switch (event.button) {
            case 'Left': {
              window.isVisible().then(
                (visible) => {
                  if (visible) {
                    window.hide().catch(console.error);
                  } else {
                    window.show().catch(console.error);
                  }
                },
                (err) => console.error(`Failed to toggle window visibility: ${err}`),
              );
              break;
            }
          }
          break;
        }
      }
    },
  };

  return await TrayIcon.new(options);
}

async function tooltip(text: string | null = null): Promise<void> {
  if (!tray) {
    console.warn('Tray icon is not initialized.');
    return;
  }
  try {
    await tray.setTooltip(text);
  } catch (e) {
    console.error(`Failed to set tray tooltip: ${text}`, e);
  }
}

export function initTrayIcon() {
  if (tray) {
    console.warn('Tray icon is already initialized.');
    return;
  }

  init()
    .then((trayIcon) => {
      tray = trayIcon;
      console.info('Tray icon initialized successfully.');
    })
    .catch((error) => {
      console.error('Failed to initialize tray icon:', error);
      throw error;
    });
}

export const useTray = () => {
  return { tooltip };
};
