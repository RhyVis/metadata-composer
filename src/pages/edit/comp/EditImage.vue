<script lang="ts" setup>
import type { UseEdit } from '@/pages/edit/script/useEdit.ts';
import { storeToRefs } from 'pinia';
import { useGlobalStore } from '@/stores/global.ts';
import { useImg } from '@/pages/edit/script/useImg.ts';

const { isDevMode } = storeToRefs(useGlobalStore());

const { edit } = defineProps<{
  edit: UseEdit;
}>();
const { editData } = edit;
const { imageSrc, inputUrl, showImage, setShowImage, pasteUrl, requestImage, clearImage } =
  useImg(edit);
</script>

<template>
  <q-card class="q-my-sm" v-if="isDevMode">
    <q-card-section>
      <div class="text-caption">Hash: {{ editData.image ?? '`undefined`' }}</div>
      <div class="text-caption">Asset: {{ imageSrc ?? '`undefined`' }}</div>
    </q-card-section>
  </q-card>
  <q-field hint="此处存储的为图片哈希值" label="图片" stack-label>
    <div>{{ editData.image ?? '拖拽文件到窗口以上传图片' }}</div>
    <template #after>
      <q-btn v-if="imageSrc" flat icon="image" size="md" square @click="setShowImage(true)" />
      <q-btn v-else flat icon="add_photo_alternate" size="md" square>
        <q-popup-proxy style="width: 500px">
          <q-card>
            <q-card-section>输入网址以从网络请求图片</q-card-section>
            <q-separator />
            <q-card-section>
              <q-input v-model="inputUrl" dense>
                <template #after>
                  <q-btn-group flat>
                    <q-btn flat icon="content_paste" round size="md" @click="pasteUrl" />
                    <q-btn
                      v-close-popup
                      flat
                      icon="add_photo_alternate"
                      round
                      size="md"
                      @click="requestImage"
                    />
                  </q-btn-group>
                </template>
              </q-input>
            </q-card-section>
          </q-card>
        </q-popup-proxy>
      </q-btn>
    </template>
  </q-field>
  <q-dialog v-model="showImage" persistent transition-hide="scale" transition-show="scale">
    <q-card class="full-width">
      <q-card-section>
        <q-img :src="imageSrc" />
      </q-card-section>
      <q-card-actions align="right">
        <q-btn-group flat>
          <q-btn flat icon="delete" size="md" @click="clearImage" />
          <q-btn flat icon="close" size="md" @click="setShowImage(false)" />
        </q-btn-group>
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>
