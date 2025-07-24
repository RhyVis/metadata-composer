import { useQuasar } from 'quasar';

export const useNotify = () => {
  const { notify } = useQuasar();

  const notifyInfo = (msg: string, caption?: string, timeout?: number) =>
    notify({
      type: 'info',
      message: msg,
      caption: caption,
      icon: 'info',
      position: 'top',
      timeout: timeout ?? 1000,
    });

  const notifySuccess = (msg: string, caption?: string, timeout?: number) =>
    notify({
      type: 'positive',
      message: msg,
      caption: caption,
      icon: 'check_circle',
      position: 'top',
      timeout: timeout ?? 1000,
    });

  const notifyWarning = (msg: string, caption?: string, timeout?: number) =>
    notify({
      type: 'warning',
      message: msg,
      caption: caption,
      icon: 'warning',
      position: 'top',
      timeout: timeout ?? 1000,
    });

  const notifyError = (msg: string, cause?: unknown, timeout?: number) =>
    notify({
      type: 'negative',
      message: msg,
      caption: cause instanceof Error ? cause.message : String(cause),
      icon: 'error',
      position: 'top',
      timeout: timeout ?? 2000,
    });

  return { notifyInfo, notifySuccess, notifyWarning, notifyError };
};
