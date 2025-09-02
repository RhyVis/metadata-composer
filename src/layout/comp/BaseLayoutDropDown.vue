<script lang="ts" setup>
import { computed } from 'vue';
import { storeToRefs } from 'pinia';
import { useI18n } from 'vue-i18n';
import { useConfigStore } from '@/stores/config';

const dev = computed(() => import.meta.env.DEV);
const { t } = useI18n();

const configStore = useConfigStore();
const { isDevMode, isDarkMode, isSafeForWorkMode } = storeToRefs(configStore);
const { toggleDevMode, toggleDarkMode, toggleSafeForWorkMode } = configStore;
</script>

<template>
  <q-btn-group flat>
    <q-btn flat icon="settings" round>
      <q-menu anchor="center middle" self="top right">
        <q-list class="r-no-sel" bordered separator>
          <q-item v-if="dev" clickable @click="toggleDevMode">
            <q-item-section avatar>
              <q-icon :name="isDevMode ? 'code' : 'code_off'" />
            </q-item-section>
            <q-item-section>{{ t('layout.drop-down.dev-mode') }}</q-item-section>
          </q-item>
          <q-item clickable @click="toggleDarkMode">
            <q-item-section avatar>
              <q-icon :name="isDarkMode ? 'dark_mode' : 'light_mode'" />
            </q-item-section>
            <q-item-section>
              {{ isDarkMode ? t('general.dark-mode') : t('general.light-mode') }}
            </q-item-section>
          </q-item>
          <q-item clickable @click="toggleSafeForWorkMode">
            <q-item-section avatar>
              <q-icon :name="isSafeForWorkMode ? 'lock' : 'favorite'" />
            </q-item-section>
            <q-item-section>
              {{ isSafeForWorkMode ? t('general.safe-for-work') : t('general.not-safe-for-work') }}
            </q-item-section>
          </q-item>
        </q-list>
      </q-menu>
    </q-btn>
  </q-btn-group>
</template>
