import type { AppConfig, Language } from '@/api/types.ts';
import { useQuasar } from 'quasar';
import { useI18n } from 'vue-i18n';
import { Command } from '@/api/cmd.ts';
import { selectDirectory } from '@/api/dialog.ts';
import { useNotify } from '@/hooks/useNotify';
import { useConfigStore } from '@/stores/config.ts';
import { message } from '@tauri-apps/plugin-dialog';
import { exit } from '@tauri-apps/plugin-process';
import { get } from '@vueuse/core';

export const LanguageList: {
  lang: Language;
  label: string;
}[] = [
  { lang: 'zh-CN', label: '简体中文' },
  { lang: 'en-US', label: 'English' },
  { lang: 'ja-JP', label: '日本語' },
];

export const useConfig = () => {
  const { t } = useI18n();
  const { sync, lang: langStore } = useConfigStore();
  const { loading } = useQuasar();
  const { notifyError, notifyWarning, notifySuccess } = useNotify();

  const handleSelectDir = async (name: keyof AppConfig) => {
    try {
      const path = await selectDirectory();
      if (!path) {
        notifyWarning(t('general.select-path-fail'), t('general.select-no-dir'));
        return;
      }
      switch (name) {
        case 'path_data': {
          await Command.configUpdate('path_data', path);
          await message(t('page.config.notify.update-restart'), {
            title: t('general.notification'),
          });
          await sync();
          await exit();
          break;
        }
        case 'path_deploy': {
          await Command.configUpdate('path_deploy', path);
          await sync();
          break;
        }
      }
    } catch (e) {
      console.error(e);
      notifyError(t('general.select-path-fail'), e);
    }
  };

  const handleClearField = async (name: keyof AppConfig) => {
    try {
      await Command.configUpdate(name, null);
      await sync();
    } catch (e) {
      console.error(e);
      notifyError(t('page.config.notify.clear-field-fail'), e);
    }
  };

  const handleClearImageCache = async () => {
    try {
      loading.show({ message: t('page.config.notify.clear-unused-image.loading') });
      const count = await Command.utilClearUnusedImages();
      if (count > 0) {
        notifySuccess(
          t('page.config.notify.clear-unused-image.success'),
          t('page.config.notify.clear-unused-image.count', [count]),
        );
      } else {
        notifySuccess(
          t('page.config.notify.clear-unused-image.success'),
          t('page.config.notify.clear-unused-image.nothing'),
        );
      }
    } catch (e) {
      console.error(e);
      notifyError(t('page.config.notify.clear-unused-image.fail'), e);
    } finally {
      loading.hide();
    }
  };

  const handleExport = async () => {
    try {
      loading.show({ message: t('page.config.notify.export.loading') });
      await Command.metadataExport();
      notifySuccess(t('page.config.notify.export.success'));
    } catch (e) {
      console.error(e);
      notifyError(t('page.config.notify.export.fail'), e);
    } finally {
      loading.hide();
    }
  };

  const handleImport = async () => {
    try {
      loading.show({ message: t('page.config.notify.import.loading') });
      await Command.metadataImport();
      notifySuccess(
        t('page.config.notify.import.success'),
        t('page.config.notify.import.refresh-for-changes'),
      );
    } catch (e) {
      console.error(e);
      notifyError(t('page.config.notify.import.fail'), e);
    } finally {
      loading.hide();
    }
  };

  const handleChangeLang = async (lang: Language) => {
    try {
      const previousLang = get(langStore);
      await Command.configUpdate('lang', lang);
      await sync();
      const currentLang = get(langStore);
      console.info(`Language changed from ${previousLang} to ${currentLang}`);
    } catch (e) {
      console.error(e);
      notifyError(t('page.config.notify.import.change-lang-fail'), e);
    }
  };

  return {
    handleSelectDir,
    handleClearField,
    handleClearImageCache,
    handleExport,
    handleImport,
    handleChangeLang,
  };
};
