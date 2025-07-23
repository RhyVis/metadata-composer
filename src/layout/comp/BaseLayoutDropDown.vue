<script lang="ts" setup>
import { computed } from 'vue';
import { storeToRefs } from 'pinia';
import { useGlobalStore } from '@/stores/global.ts';

const dev = computed(() => import.meta.env.DEV);
const { isDevMode, isDarkMode } = storeToRefs(useGlobalStore());
</script>

<template>
  <q-btn-group flat>
    <q-btn flat icon="settings" round>
      <q-menu anchor="center middle" self="top right">
        <q-list class="r-no-sel" bordered separator>
          <q-item v-if="dev" clickable @click="isDevMode = !isDevMode">
            <q-item-section avatar>
              <q-icon :name="isDevMode ? 'code' : 'code_off'" />
            </q-item-section>
            <q-item-section>调试模式</q-item-section>
          </q-item>
          <q-item clickable @click="isDarkMode = !isDarkMode">
            <q-item-section avatar>
              <q-icon :name="isDarkMode ? 'dark_mode' : 'light_mode'" />
            </q-item-section>
            <q-item-section>
              {{ isDarkMode ? '暗黑模式' : '亮色模式' }}
            </q-item-section>
          </q-item>
        </q-list>
      </q-menu>
    </q-btn>
  </q-btn-group>
</template>
