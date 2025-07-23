import type { DecompressionInfoPayload } from '@/api/event.ts';
import type { UnlistenFn } from '@tauri-apps/api/event';
import { useQuasar } from 'quasar';
import { Command } from '@/api/cmd.ts';
import { selectDirectory } from '@/api/dialog.ts';
import { truncateString } from '@/api/util.ts';
import { useNotify } from '@/composables/useNotify.ts';
import { useTray } from '@/composables/useTray.ts';
import { useLibraryStore } from '@/stores/library.ts';
import { useTableStore } from '@/stores/table.ts';
import { listen } from '@tauri-apps/api/event';

export const useOperation = () => {
  const { fetch } = useLibraryStore();
  const { syncDeploymentCache } = useTableStore();
  const { loading } = useQuasar();
  const { notifySuccess, notifyError } = useNotify();
  const { tooltip } = useTray();

  const handleReload = async () => {
    console.info('Reloading table data...');
    loading.show();
    try {
      await fetch();
      notifySuccess('数据已刷新');
    } catch (e) {
      console.error(e);
      notifyError('数据刷新失败', e);
    } finally {
      loading.hide();
    }
  };

  const handleRemove = async (id: string) => {
    console.info(`Removing item with id: ${id}`);
    loading.show();
    try {
      await Command.metadataDelete(id);
      await fetch();
      notifySuccess(`已成功删除 '${id}'`);
    } catch (e) {
      console.error(e);
      notifyError(`删除 '${id}' 失败`, e);
    } finally {
      loading.hide();
    }
  };

  const handleDeploy = async (id: string, useDeployDir: boolean) => {
    console.info(`Deploying item with id: ${id}`);

    if (useDeployDir) {
      let eventHandle: UnlistenFn | undefined;
      try {
        const msg = `正在部署 '${id}' 到设置目录...`;
        loading.show({
          message: msg,
        });
        await tooltip(msg);

        listen<DecompressionInfoPayload>('decompression_progress', (event) => {
          const progress = event.payload[0];
          const fileCount = event.payload[1];
          const currentFile = truncateString(event.payload[2], 25);
          loading.show({
            message: `${msg}<br>解压进度：${progress}%<br>文件数量：${fileCount}<br>当前文件：${currentFile}`,
            html: true,
          });
          tooltip(
            `${msg}\n解压进度：${progress}%\n文件数量：${fileCount}\n当前文件：${currentFile}`,
          ).catch(console.error);
        }).then(
          (handle) => (eventHandle = handle),
          (error) => console.error(`Failed to listen for decompression progress: ${error}`),
        );

        await Command.metadataDeploy(id, {
          use_config_dir: true,
          target_dir: null,
        });
        await fetch();
        syncDeploymentCache();

        notifySuccess(`已成功部署 '${id}' 到设置目录`);
      } catch (e) {
        console.error(e);
        notifyError(`部署 '${id}' 失败`, e);
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
            const msg = `正在部署 '${id}' 到 ${path}...`;
            loading.show({
              message: msg,
            });
            await tooltip(msg);

            listen<number>('decompression_progress', (event) => {
              loading.show({
                message: `${msg}<br>解压进度: ${event.payload}%`,
                html: true,
              });
              tooltip(`${msg}\n解压进度: ${event.payload}%`).catch(console.error);
            }).then(
              (handle) => (eventHandle = handle),
              (error) => console.error(`Failed to listen for decompression progress: ${error}`),
            );

            await Command.metadataDeploy(id, {
              use_config_dir: false,
              target_dir: path,
            });
            await fetch();
            syncDeploymentCache();

            notifySuccess(`已成功部署 '${id}' 到 ${path}`);
          } catch (e) {
            console.error(e);
            notifyError(`部署 '${id}' 失败`, e);
          } finally {
            loading.hide();
            eventHandle?.();
            await tooltip();
          }
        } else {
          notifyError('部署取消', '未选择有效的目录');
        }
      } catch (e) {
        console.error(e);
        notifyError('选择目录失败', e);
      }
    }
  };

  const handleDeployOff = async (id: string) => {
    console.info(`Un-deploying item with id: ${id}`);
    loading.show({
      message: `正在取消部署 '${id}'...`,
    });
    try {
      await Command.metadataDeployOff(id);
      await fetch();
      syncDeploymentCache();

      notifySuccess(`已成功取消部署 '${id}'`);
    } catch (e) {
      console.error(e);
      notifyError(`取消部署 '${id}' 失败`, e);
    } finally {
      loading.hide();
    }
  };

  return { handleReload, handleRemove, handleDeploy, handleDeployOff };
};
