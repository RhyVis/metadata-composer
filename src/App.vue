<script lang="ts" setup>
import { onMounted } from 'vue';
import ContextMenu from '@/layout/ContextMenu.vue';
import { useInit } from '@/hooks/useInit';

const { init } = useInit();

onMounted(() => init());
</script>

<template>
  <div>
    <RouterView />
    <ContextMenu />
  </div>
</template>
