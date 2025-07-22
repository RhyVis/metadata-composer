import type { InternalConfig } from '@/api/types.ts';
import { useQuasar } from 'quasar';
import { Command } from '@/api/cmd.ts';
import { selectDirectory } from '@/api/dialog.ts';
import { useNotify } from '@/composables/useNotify.ts';
import { useConfigStore } from '@/stores/config.ts';

export const useConfig = () => {
  const { sync } = useConfigStore();
  const { loading } = useQuasar();
  const { notifyError, notifyWarning, notifySuccess } = useNotify();

  const handleSelectDir = async (name: keyof InternalConfig) => {
    try {
      const path = await selectDirectory();
      if (!path) {
        notifyWarning('选择路径失败', '未选择任何目录');
        return;
      }
      switch (name) {
        case 'root_data': {
          await Command.configUpdate('root_data', path);
          await sync();
          break;
        }
        case 'root_deploy': {
          await Command.configUpdate('root_deploy', path);
          await sync();
          break;
        }
      }
    } catch (e) {
      console.error(e);
      notifyError('选择路径失败', e);
    }
  };

  const handleClearField = async (name: keyof InternalConfig) => {
    try {
      await Command.configUpdate(name, undefined);
      await sync();
    } catch (e) {
      console.error(e);
      notifyError('清除属性失败', e);
    }
  };

  const handleClearImageCache = async () => {
    try {
      loading.show({ message: '正在清除图片缓存...' });
      const count = await Command.utilClearUnusedImages();
      if (count > 0) {
        notifySuccess('清除图片缓存成功', `已清除 ${count} 张未使用的图片`);
      } else {
        notifySuccess('清除图片缓存成功', '没有未使用的图片');
      }
    } catch (e) {
      console.error(e);
      notifyError('清除图片缓存失败', e);
    } finally {
      loading.hide();
    }
  };

  const handleExport = async () => {
    try {
      loading.show({ message: '正在导出元数据...' });
      await Command.metadataExport();
      notifySuccess('导出元数据成功');
    } catch (e) {
      console.error(e);
      notifyError('导出元数据失败', e);
    } finally {
      loading.hide();
    }
  };

  const handleImport = async () => {
    try {
      loading.show({ message: '正在导入元数据...' });
      await Command.metadataImport();
      notifySuccess('导入元数据成功', '刷新数据库以查看更改');
    } catch (e) {
      console.error(e);
      notifyError('导入元数据失败', e);
    } finally {
      loading.hide();
    }
  };

  return {
    handleSelectDir,
    handleClearField,
    handleClearImageCache,
    handleExport,
    handleImport,
  };
};
