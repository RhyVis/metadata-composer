<script lang="ts" setup>
import type { UseEdit } from '@/pages/edit/script/useEdit.ts';
import { storeToRefs } from 'pinia';
import { useGlobalStore } from '@/stores/global.ts';
import { useImg } from '@/pages/edit/script/useImg.ts';
import { useDarkStyle } from '@/composables/useDarkStyle.ts';

const { isDevMode } = storeToRefs(useGlobalStore());
const { textClass } = useDarkStyle();

const { edit } = defineProps<{
  edit: UseEdit;
}>();
const { editData } = edit;
const { imageSrc, showImage, setShowImage, pasteImg, clearImage } = useImg(edit);
</script>

<template>
  <q-card class="q-my-sm" v-if="isDevMode">
    <q-card-section>
      <div class="text-caption">Hash: {{ editData.image ?? '`undefined`' }}</div>
      <div class="text-caption">Asset: {{ imageSrc ?? '`undefined`' }}</div>
    </q-card-section>
  </q-card>
  <q-field hint="此处存储的为图片哈希值" label="图片" stack-label>
    <div :class="textClass">
      {{
        editData.image ??
        '拖拽文件到窗口以上传图片，支持PNG、JPG、JPEG 或 WEBP格式；或点击右侧按钮粘贴图片或图片链接'
      }}
    </div>
    <template #after>
      <q-btn v-if="imageSrc" flat icon="image" round size="md" @click="setShowImage(true)">
        <q-tooltip>查看图片详情</q-tooltip>
      </q-btn>
      <q-btn v-else flat icon="add_photo_alternate" round size="md" @click="pasteImg">
        <q-tooltip>从剪贴板读取图片或图片链接</q-tooltip>
      </q-btn>
    </template>
  </q-field>
  <q-dialog v-model="showImage" persistent transition-hide="scale" transition-show="scale">
    <q-card class="full-width">
      <q-card-section>
        <q-img :src="imageSrc" />
      </q-card-section>
      <q-card-actions align="right">
        <div class="text-caption q-ml-sm">{{ editData.image }}</div>
        <q-space />
        <q-btn-group flat rounded>
          <q-btn flat icon="delete" rounded size="md" @click="clearImage" />
          <q-btn flat icon="close" rounded size="md" @click="setShowImage(false)" />
        </q-btn-group>
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>
