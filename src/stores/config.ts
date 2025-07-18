import type { InternalConfig } from '@/api/types.ts';
import { defineStore } from 'pinia';
import { Command } from '@/api/cmd.ts';

export const useConfigStore = defineStore('config', {
  state: (): InternalConfig => ({
    root_data: '',
    root_deploy: null,
  }),
  getters: {
    hasDeployRoot: (state) => !!state.root_deploy,
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
