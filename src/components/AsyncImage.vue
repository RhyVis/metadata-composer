<script lang="ts" setup>
import { ref, watch } from 'vue';
import { set } from '@vueuse/core';
import { resolveImageAsset } from '@/api/util.ts';
import { storeToRefs } from 'pinia';
import { useTableStore } from '@/pages/main/script/useTableStore';

const { hash } = defineProps<{
  hash: string | undefined;
}>();
const assetUrl = ref('');

const loading = ref(false);
const error = ref<string | undefined>(undefined);

const { imageCache } = storeToRefs(useTableStore());

const resolve = async (hash: string | undefined) => {
  if (!hash) {
    set(assetUrl, '');
    return;
  }
  if (imageCache.value[hash as keyof typeof imageCache.value]) {
    set(assetUrl, imageCache.value[hash as keyof typeof imageCache.value]);
    return;
  }

  set(loading, true);
  set(error, undefined);

  try {
    const url = await resolveImageAsset(hash);
    if (url) {
      set(assetUrl, url);
      imageCache.value[hash as keyof typeof imageCache.value] = url;
    }
  } catch (e) {
    console.error(e);
    set(assetUrl, '');
    set(error, e instanceof Error ? e.message : String(e));
  } finally {
    set(loading, false);
  }
};

watch(
  () => hash,
  (newHash) => {
    resolve(newHash);
  },
  { immediate: true },
);
</script>

<template>
  <slot name="state" :asset-url="assetUrl" :error="error" :loading="loading" />
</template>
