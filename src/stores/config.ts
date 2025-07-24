import type { AppConfig } from '@/api/types.ts';
import { defineStore } from 'pinia';
import { computed, ref } from 'vue';
import { Command } from '@/api/cmd.ts';
import { set } from '@vueuse/core';

interface FrontendConfig {
  devMode: boolean;
  darkMode: boolean;
}

export const useConfigStore = defineStore(
  'config',
  () => {
    const frontend = ref<FrontendConfig>({
      devMode: false,
      darkMode: false,
    });
    const backend = ref<AppConfig>({
      path_data: '',
      path_deploy: null,
    });

    const isDevMode = computed(() => frontend.value.devMode);
    const isDarkMode = computed(() => frontend.value.darkMode);

    const pathData = computed(() => backend.value.path_data);
    const pathDeploy = computed(() => backend.value.path_deploy);

    async function sync() {
      set(backend, await Command.configGet());
    }

    function toggleDevMode() {
      frontend.value.devMode = !frontend.value.devMode;
    }

    function toggleDarkMode() {
      frontend.value.darkMode = !frontend.value.darkMode;
    }

    function setDataPath(path: string) {
      backend.value.path_data = path;
    }

    function setDeployPath(path: string | null) {
      backend.value.path_deploy = path;
    }

    return {
      // State
      frontend,
      backend,
      // Getter
      isDevMode,
      isDarkMode,
      pathData,
      pathDeploy,
      // Action
      sync,
      toggleDevMode,
      toggleDarkMode,
      setDataPath,
      setDeployPath,
    };
  },
  {
    tauri: {
      filterKeys: 'frontend',
      filterKeysStrategy: 'pick',
    },
  },
);
