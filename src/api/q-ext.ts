import type { QNotifyCreateOptions } from 'quasar';

export function notifySuccess(msg: string, caption?: string): QNotifyCreateOptions {
  return {
    type: 'positive',
    message: msg,
    caption: caption,
    icon: 'check_circle',
    position: 'top',
  };
}

export function notifyWarning(msg: string, caption?: string): QNotifyCreateOptions {
  return {
    type: 'warning',
    message: msg,
    caption: caption,
    icon: 'warning',
    position: 'top',
  };
}

export function notifyError(msg: string, cause?: unknown): QNotifyCreateOptions {
  return {
    type: 'negative',
    message: msg,
    caption: cause as string,
    icon: 'error',
    position: 'top',
  };
}
