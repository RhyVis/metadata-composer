import type { WatchHandle } from 'vue';
import { storeToRefs } from 'pinia';
import { useQuasar } from 'quasar';
import { onUnmounted, watch } from 'vue';
import { useI18n } from 'vue-i18n';
import { useTableStore } from '@/pages/main/script/useTableStore';
import { useConfigStore } from '@/stores/config';
import { useDatabaseStore } from '@/stores/database';
import { get, syncRef } from '@vueuse/core';

export const useInit = () => {
  const { locale } = useI18n();
  const tableStore = useTableStore();
  const configStore = useConfigStore();
  const databaseStore = useDatabaseStore();

  const { isDarkMode, lang } = storeToRefs(configStore);
  const { dark } = useQuasar();

  let watchHandle: WatchHandle | undefined;

  const init = () => {
    tableStore.$tauri.start().catch(console.error);
    configStore.$tauri
      .start()
      .then(() => {
        configStore.sync().catch(console.error);
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
      .catch(console.error);
    databaseStore.sync().catch(console.error);
  };

  onUnmounted(() => watchHandle?.());

  return { init };
};
