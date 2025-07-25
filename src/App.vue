<script lang="ts" setup>
import { onMounted } from 'vue';
import ContextMenu from '@/layout/ContextMenu.vue';
import { useInit } from '@/hooks/useInit';
import { storeToRefs } from 'pinia';
import { useConfigStore } from '@/stores/config';

const { init } = useInit();
const { lang } = storeToRefs(useConfigStore());

onMounted(() => init());
</script>

<template>
  <div>
    <RouterView />
    <ContextMenu :key="lang" />
  </div>
</template>
