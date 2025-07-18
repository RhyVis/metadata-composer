import { debug, error, info, trace, warn } from '@tauri-apps/plugin-log';

export function initLogger() {
  function forwardConsole(
    fnName: 'log' | 'debug' | 'info' | 'warn' | 'error',
    logger: (message: string) => Promise<void>,
  ) {
    const original = console[fnName];
    console[fnName] = (message) => {
      original(message);
      logger(message).catch((err) => original(err));
    };
  }

  forwardConsole('log', trace);
  forwardConsole('debug', debug);
  forwardConsole('info', info);
  forwardConsole('warn', warn);
  forwardConsole('error', error);
}

export function initKeyBlocker() {
  window.addEventListener('keydown', (event) => {
    if (event.key === 'F5' || (event.ctrlKey && event.key === 'r')) {
      event.preventDefault();
    }
  });
}
