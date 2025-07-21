import { storeToRefs } from 'pinia';
import { computed } from 'vue';
import { useGlobalStore } from '@/stores/global.ts';

export const useDarkStyle = () => {
  const { isDarkMode } = storeToRefs(useGlobalStore());

  const textClass = computed(() => (isDarkMode.value ? 'text-white' : 'text-dark'));

  return { textClass };
};
