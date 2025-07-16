import type { UseEdit } from '@/pages/edit/script/useEdit.ts';
import { ref } from 'vue';
import { ContentTypeEnum } from '@/pages/edit/script/define.ts';

export const useContentInfo = (edit: UseEdit) => {
  const currentType = ref(ContentTypeEnum.Undefined);

  return { currentType };
};
