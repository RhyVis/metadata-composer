<script lang="ts" setup>
import Icon from '@/assets/icon-square.svg';
import { useQuasar } from 'quasar';
import { computed } from 'vue';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { storeToRefs } from 'pinia';
import { useGlobalStore } from '@/stores/global.ts';
import { useRouter } from 'vue-router';

const dev = computed(() => import.meta.env.DEV);
const window = getCurrentWindow();
const { dark } = useQuasar();
const { push } = useRouter();
const { isDevMode } = storeToRefs(useGlobalStore());

const headerClass = computed(() =>
  dark.isActive ? ['bg-dark', 'text-white'] : ['bg-white', 'text-blue-grey-9'],
);
const contentClass = computed(() =>
  dark.isActive ? ['bg-dark', 'text-white'] : ['bg-grey-1', 'text-blue-grey-9'],
);
</script>

<template>
  <q-layout view="hHh lpR fFf">
    <q-header class="region-drag" :class="headerClass" bordered>
      <q-toolbar>
        <q-toolbar-title>
          <div class="row items-center r-no-sel">
            <q-avatar>
              <img :src="Icon" alt="C" />
            </q-avatar>
            <span class="q-pl-sm text-bold">Composer</span>
          </div>
          <q-space />
        </q-toolbar-title>

        <div class="row region-none">
          <q-btn-group flat>
            <q-btn
              v-if="dev"
              :icon="isDevMode ? 'code' : 'code_off'"
              flat
              push
              round
              @click="isDevMode = !isDevMode"
            />
            <q-btn
              :icon="dark.isActive ? 'dark_mode' : 'light_mode'"
              flat
              push
              round
              @click="dark.toggle()"
            />
            <q-btn flat icon="settings" push round @click="push('config')" />
          </q-btn-group>
          <q-separator class="q-mx-xs" inset vertical />
          <q-btn-group flat push>
            <q-btn flat icon="minimize" push size="sm" @click="window.minimize()" />
            <q-btn flat icon="maximize" push size="sm" @click="window.maximize()" />
            <q-btn flat icon="close" push size="sm" @click="window.close()" />
          </q-btn-group>
        </div>
      </q-toolbar>
    </q-header>

    <q-page-container :class="contentClass">
      <router-view />
    </q-page-container>
  </q-layout>
</template>
