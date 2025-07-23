import type { AppConfig } from '@/api/types.ts';
import { defineStore } from 'pinia';
import { Command } from '@/api/cmd.ts';

export const useConfigStore = defineStore('config', {
  state: (): AppConfig => ({
    path_data: '',
    path_deploy: null,
  }),
  getters: {
    hasDeployPath: (state) => !!state.path_deploy,
  },
  actions: {
    async sync() {
      try {
        this.$patch(await Command.configGet());
      } catch (e) {
        console.error(`Config sync error: ${e}`);
        throw e;
      }
    },
  },
});
