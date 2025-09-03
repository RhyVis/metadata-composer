import { storeToRefs } from 'pinia';
import { computed } from 'vue';
import { useConfigStore } from '@/stores/config';

export const useDarkStyle = () => {
  const { isDarkMode } = storeToRefs(useConfigStore());

  const textClass = computed(() => (isDarkMode.value ? 'text-white' : 'text-dark'));

  return { textClass };
};
