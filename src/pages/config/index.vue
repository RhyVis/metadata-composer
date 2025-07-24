<script lang="ts" setup>
import { useConfigStore } from '@/stores/config.ts';
import { storeToRefs } from 'pinia';
import { Command } from '@/api/cmd.ts';
import { useConfig } from '@/pages/config/script/useConfig.ts';

const config = useConfigStore();
const { pathData, pathDeploy } = storeToRefs(config);
const { handleSelectDir, handleClearField, handleClearImageCache, handleExport, handleImport } =
  useConfig();
</script>

<template>
  <q-page padding>
    <q-list bordered padding>
      <q-item-label header>{{ $t('page.config.storage.header') }}</q-item-label>

      <q-item v-ripple>
        <q-item-section>
          <q-item-label>{{ $t('page.config.storage.path-data') }}</q-item-label>
          <q-item-label caption>{{ pathData || $t('general.unset') }}</q-item-label>
        </q-item-section>
        <q-item-section side top>
          <q-btn-group flat>
            <q-btn flat icon="folder" round @click="handleSelectDir('path_data')" />
          </q-btn-group>
        </q-item-section>
      </q-item>

      <q-item v-ripple>
        <q-item-section>
          <q-item-label>{{ $t('page.config.storage.path-deploy') }}</q-item-label>
          <q-item-label caption>{{ pathDeploy || $t('general.unset') }}</q-item-label>
        </q-item-section>
        <q-item-section side top>
          <q-btn-group flat>
            <q-btn
              v-if="pathDeploy"
              flat
              icon="clear"
              round
              @click="handleClearField('path_deploy')"
            />
            <q-btn flat icon="folder" round @click="handleSelectDir('path_deploy')" />
          </q-btn-group>
        </q-item-section>
      </q-item>

      <q-item v-ripple clickable @click="handleClearImageCache">
        <q-item-section>
          <q-item-label>{{ $t('page.config.storage.clear-image-cache') }}</q-item-label>
          <q-item-label caption>
            {{ $t('page.config.storage.clear-image-cache-desc') }}
          </q-item-label>
        </q-item-section>
      </q-item>

      <q-item-label header>{{ $t('page.config.database.header') }}</q-item-label>

      <q-item v-ripple clickable @click="handleExport">
        <q-item-section>
          <q-item-label>{{ $t('page.config.database.export.label') }}</q-item-label>
          <q-item-label caption>{{ $t('page.config.database.export.desc') }}</q-item-label>
        </q-item-section>
      </q-item>

      <q-item v-ripple clickable @click="handleImport">
        <q-item-section>
          <q-item-label>{{ $t('page.config.database.import.label') }}</q-item-label>
          <q-item-label caption>{{ $t('page.config.database.import.desc') }}</q-item-label>
        </q-item-section>
      </q-item>

      <q-item-label header>{{ $t('page.config.directory.header') }}</q-item-label>

      <q-item v-ripple clickable @click="Command.openConfigDir()">
        <q-item-section>
          <q-item-label>{{ $t('page.config.directory.dir-config.label') }}</q-item-label>
          <q-item-label caption> {{ $t('page.config.directory.dir-config.desc') }} </q-item-label>
        </q-item-section>
      </q-item>

      <q-item v-ripple clickable @click="Command.openLogDir()">
        <q-item-section>
          <q-item-label>{{ $t('page.config.directory.dir-log.label') }}</q-item-label>
          <q-item-label caption> {{ $t('page.config.directory.dir-log.desc') }} </q-item-label>
        </q-item-section>
      </q-item>
    </q-list>
  </q-page>
</template>
