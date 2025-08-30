import type { DecompressionInfoPayload } from '@/api/event.ts';
import type { UnlistenFn } from '@tauri-apps/api/event';
import { useQuasar } from 'quasar';
import { useI18n } from 'vue-i18n';
import { Command } from '@/api/cmd.ts';
import { selectDirectory } from '@/api/dialog.ts';
import { truncateString } from '@/api/util.ts';
import { useNotify } from '@/hooks/useNotify';
import { useTray } from '@/hooks/useTray';
import { useTableStore } from '@/pages/main/script/useTableStore';
import { useDatabaseStore } from '@/stores/database';
import { listen } from '@tauri-apps/api/event';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { sendNotification } from '@tauri-apps/plugin-notification';

export type UseOperation = ReturnType<typeof useOperation>;

export const useOperation = () => {
  const { t } = useI18n();
  const { sync } = useDatabaseStore();
  const { syncDeploymentCache } = useTableStore();
  const { loading } = useQuasar();
  const { notifySuccess, notifyError } = useNotify();
  const { tooltip } = useTray();

  const window = getCurrentWindow();

  const handleReload = async () => {
    console.info('Reloading table data...');
    loading.show();
    try {
      await sync();
      notifySuccess(t('page.main.notify.refresh.success'));
    } catch (e) {
      console.error(e);
      notifyError(t('page.main.notify.refresh.fail'), e);
    } finally {
      loading.hide();
    }
  };

  const handleRemove = async (id: string) => {
    console.info(`Removing item with id: ${id}`);
    loading.show();
    try {
      await Command.metadataDelete(id);
      await sync();
      notifySuccess(t('page.main.notify.delete.success', [id]));
    } catch (e) {
      console.error(e);
      notifyError(t('page.main.notify.delete.fail', [id]), e);
    } finally {
      loading.hide();
    }
  };

  const handleDeploy = async (id: string, useDeployDir: boolean) => {
    console.info(`Deploying item with id: ${id}`);

    if (useDeployDir) {
      let eventHandle: UnlistenFn | undefined;
      try {
        const msg = t('page.main.loading.deploying-to-configured', [id]);
        loading.show({
          message: msg,
        });
        await tooltip(msg);

        listen<DecompressionInfoPayload>('decompression_progress', (event) => {
          const progress = event.payload[0];
          const fileCount = event.payload[1];
          const currentFile = truncateString(event.payload[2], 25);
          loading.show({
            message: t('page.main.loading.decompress-progress', [
              msg,
              progress,
              fileCount,
              currentFile,
            ]),
            html: true,
          });
          tooltip(
            t('page.main.notify.decompress-progress', [msg, progress, fileCount, currentFile]),
          ).catch(console.error);
        }).then(
          (handle) => (eventHandle = handle),
          (error) => console.error(`Failed to listen for decompression progress: ${error}`),
        );

        await Command.metadataDeploy(id, {
          use_config_dir: true,
          target_dir: null,
        });
        await sync();
        await syncDeploymentCache();

        const successMsg = t('page.main.notify.deploy.config-success', [id]);
        notifySuccess(successMsg);
        if (!(await window.isFocused())) sendNotification(successMsg);
      } catch (e) {
        console.error(e);
        notifyError(t('page.main.notify.deploy.config-fail', [id]), e);
      } finally {
        loading.hide();
        eventHandle?.();
        await tooltip();
      }
    } else {
      try {
        const path = await selectDirectory();
        if (path) {
          let eventHandle: UnlistenFn | undefined;
          try {
            const msg = t('page.main.loading.deploying-to-custom', [id, path]);
            loading.show({
              message: msg,
            });
            await tooltip(msg);

            listen<DecompressionInfoPayload>('decompression_progress', (event) => {
              const progress = event.payload[0];
              const fileCount = event.payload[1];
              const currentFile = truncateString(event.payload[2], 25);
              loading.show({
                message: t('page.main.loading.decompress-progress', [
                  msg,
                  progress,
                  fileCount,
                  currentFile,
                ]),
                html: true,
              });
              tooltip(
                t('page.main.notify.decompress-progress', [msg, progress, fileCount, currentFile]),
              ).catch(console.error);
            }).then(
              (handle) => (eventHandle = handle),
              (error) => console.error(`Failed to listen for decompression progress: ${error}`),
            );

            await Command.metadataDeploy(id, {
              use_config_dir: false,
              target_dir: path,
            });
            await sync();
            await syncDeploymentCache();

            notifySuccess(t('page.main.notify.deploy.custom-success', [id, path]));
          } catch (e) {
            console.error(e);
            notifyError(t('page.main.notify.deploy.custom-fail', [id]), e);
          } finally {
            loading.hide();
            eventHandle?.();
            await tooltip();
          }
        } else {
          notifyError(
            t('page.main.notify.deploy.cancel'),
            t('page.main.notify.deploy.cancel-no-valid-dir'),
          );
        }
      } catch (e) {
        console.error(e);
        notifyError(t('notify.select-path.fail'), e);
      }
    }
  };

  const handleDeployOff = async (id: string) => {
    console.info(`Un-deploying item with id: ${id}`);
    loading.show({
      message: t('page.main.loading.deploy-off', [id]),
    });
    try {
      await Command.metadataDeployOff(id);
      await sync();
      await syncDeploymentCache();

      notifySuccess(t('page.main.notify.deploy-off.success', [id]));
    } catch (e) {
      console.error(e);
      notifyError(t('page.main.notify.deploy-off.fail', [id]), e);
    } finally {
      loading.hide();
    }
  };

  return { handleReload, handleRemove, handleDeploy, handleDeployOff };
};
