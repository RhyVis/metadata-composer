import type { Image } from '@tauri-apps/api/image';
import type { TrayIconOptions } from '@tauri-apps/api/tray';
import { defaultWindowIcon } from '@tauri-apps/api/app';
import { Menu } from '@tauri-apps/api/menu/menu';
import { TrayIcon } from '@tauri-apps/api/tray';
import { getCurrentWindow } from '@tauri-apps/api/window';

export async function initTrayIcon(): Promise<TrayIcon> {
  const window = getCurrentWindow();

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

export const Tray = await initTrayIcon();
