import { defineStore } from 'pinia';

export const useGlobalStore = defineStore('global', {
  state: () => ({
    isDevMode: false,
    isDarkMode: false,
    enableAutoHide: false,
  }),
});
