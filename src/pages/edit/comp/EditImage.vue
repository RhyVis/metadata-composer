<script lang="ts" setup>
import type { UseEdit } from '@/pages/edit/script/useEdit.ts';
import { onMounted, ref, watch } from 'vue';
import { Command } from '@/api/cmd.ts';
import { convertFileSrc } from '@tauri-apps/api/core';
import { set, useToggle } from '@vueuse/core';
import { storeToRefs } from 'pinia';
import { useGlobalStore } from '@/stores/global.ts';

const { isDevMode } = storeToRefs(useGlobalStore());

const { edit } = defineProps<{
  edit: UseEdit;
}>();
const { editData } = edit;
const imageSrc = ref(undefined);

const [showImage, setShowImage] = useToggle(false);

const updateAssetUrl = async (hash: string | null) => {
  if (hash) {
    try {
      const path = await Command.pathResolveImg(hash);
      const assetUrl = convertFileSrc(path);
      set(imageSrc, assetUrl);
    } catch (e) {
      console.error(e);
    }
  } else {
    set(imageSrc, undefined);
  }
};

watch(
  () => editData.value.image,
  async (newHash) => await updateAssetUrl(newHash),
);
onMounted(async () => {
  await updateAssetUrl(editData.value.image);
});
</script>

<template>
  <q-card class="q-my-sm" v-if="isDevMode">
    <q-card-section>
      <div class="text-caption">Hash: {{ editData.image ?? '`undefined`' }}</div>
      <div class="text-caption">Asset: {{ imageSrc ?? '`undefined`' }}</div>
    </q-card-section>
  </q-card>
  <q-field hint="此处存储的为图片哈希值" label="图片" stack-label>
    {{ editData.image }}
    <template v-if="imageSrc" #after>
      <q-btn flat icon="image" size="md" square @click="setShowImage(true)" />
    </template>
  </q-field>
  <q-dialog v-model="showImage" persistent transition-hide="scale" transition-show="scale">
    <q-card class="full-width">
      <q-card-section>
        <q-img :src="imageSrc" />
      </q-card-section>
      <q-card-actions>
        <q-space />
        <q-btn flat icon="close" size="md" square @click="setShowImage(false)" />
      </q-card-actions>
    </q-card>
  </q-dialog>
</template>
