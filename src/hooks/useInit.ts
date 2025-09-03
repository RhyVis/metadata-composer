import type { WatchHandle } from 'vue';
import { storeToRefs } from 'pinia';
import { useQuasar } from 'quasar';
import { onUnmounted, ref, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { useTableStore } from '@/pages/main/script/useTableStore';
import { useConfigStore } from '@/stores/config';
import { useDatabaseStore } from '@/stores/database';
import { message } from '@tauri-apps/plugin-dialog';
import { exit } from '@tauri-apps/plugin-process';
import { get, syncRef } from '@vueuse/core';

export const useInit = () => {
  const { loading } = useQuasar();
  const { locale, t } = useI18n();
  const tableStore = useTableStore();
  const configStore = useConfigStore();
  const databaseStore = useDatabaseStore();

  const { isDarkMode, lang } = storeToRefs(configStore);
  const { dark } = useQuasar();

  const error = ref<string[]>([]);

  let watchHandle: WatchHandle | undefined;

  const init = async () => {
    const initTasks = [
      tableStore.$tauri
        .start()
        .then(() => {
          tableStore.syncDeploymentCache().catch((e) => {
            console.error('Failed to sync deployment cache:', e);
            error.value.push('Failed to sync deployment cache');
            throw e;
          });
        })
        .catch((e) => {
          console.error('Failed to start table store:', e);
          error.value.push('Failed to start table store');
          throw e;
        }),
      configStore.$tauri
        .start()
        .then(() => {
          configStore.sync().catch((e) => {
            console.error('Failed to sync config store:', e);
            error.value.push('Failed to sync config store');
            throw e;
          });
          dark.set(get(isDarkMode));
          watch(isDarkMode, (darkMode) => {
            if (darkMode) {
              dark.set(true);
              document.documentElement.classList.add('dark-style');
            } else {
              dark.set(false);
              document.documentElement.classList.remove('dark-style');
            }
          });
          syncRef(locale, lang, {
            direction: 'rtl',
          });
        })
        .catch((e) => {
          console.error('Failed to start config store:', e);
          error.value.push('Failed to start config store');
          throw e;
        }),
      databaseStore.sync().catch((e) => {
        console.error('Failed to sync database store:', e);
        error.value.push('Failed to sync database store');
        throw e;
      }),
    ];

    try {
      loading.show({
        message: t('general.initializing'),
      });
      await Promise.allSettled(initTasks);
      if (error.value.length > 0) {
        await message(error.value.join('\n'), {
          title: t('general.initialization-fail'),
          kind: 'error',
        });
        await exit(1);
      }
    } catch (e) {
      console.error('Initialization failed:', e);
    } finally {
      loading.hide();
    }
  };

  onUnmounted(() => watchHandle?.());

  return { init };
};
