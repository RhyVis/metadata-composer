<script lang="ts" setup>
import { useConfigStore } from '@/stores/config.ts';
import { onMounted } from 'vue';
import { useNotify } from '@/composables/useNotify.ts';
import { storeToRefs } from 'pinia';
import { Command } from '@/api/cmd.ts';
import { useConfig } from '@/pages/config/script/useConfig.ts';

const config = useConfigStore();
const { path_data, path_deploy } = storeToRefs(config);
const { notifyError } = useNotify();
const { handleSelectDir, handleClearField, handleClearImageCache, handleExport, handleImport } =
  useConfig();

onMounted(() =>
  config.sync().then(
    () => console.info('Successfully synced config file'),
    (e) => notifyError('配置同步失败', e),
  ),
);
</script>

<template>
  <q-page padding>
    <q-list bordered padding>
      <q-item-label header>存储</q-item-label>

      <q-item v-ripple>
        <q-item-section>
          <q-item-label>存储根</q-item-label>
          <q-item-label caption>{{ path_data || '未设置' }}</q-item-label>
        </q-item-section>
        <q-item-section side top>
          <q-btn-group flat>
            <q-btn flat icon="folder" round @click="handleSelectDir('path_data')" />
          </q-btn-group>
        </q-item-section>
      </q-item>

      <q-item v-ripple>
        <q-item-section>
          <q-item-label>部署根</q-item-label>
          <q-item-label caption>{{ path_deploy || '未设置' }}</q-item-label>
        </q-item-section>
        <q-item-section side top>
          <q-btn-group flat>
            <q-btn
              v-if="path_deploy"
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
          <q-item-label>清除图片缓存</q-item-label>
          <q-item-label caption>清除未使用的图片缓存</q-item-label>
        </q-item-section>
      </q-item>

      <q-item-label header>数据库</q-item-label>

      <q-item v-ripple clickable @click="handleExport">
        <q-item-section>
          <q-item-label>导出元数据</q-item-label>
          <q-item-label caption>导出当前数据库的元数据到 'lib.json'</q-item-label>
        </q-item-section>
      </q-item>

      <q-item v-ripple clickable @click="handleImport">
        <q-item-section>
          <q-item-label>导入元数据</q-item-label>
          <q-item-label caption>从 'lib.json' 导入元数据到当前数据库</q-item-label>
        </q-item-section>
      </q-item>

      <q-item-label header>目录</q-item-label>

      <q-item v-ripple clickable @click="Command.openConfigDir()">
        <q-item-section>
          <q-item-label>打开配置目录</q-item-label>
          <q-item-label caption> 配置文件为 Config.toml </q-item-label>
        </q-item-section>
      </q-item>

      <q-item v-ripple clickable @click="Command.openLogDir()">
        <q-item-section>
          <q-item-label>打开日志目录</q-item-label>
          <q-item-label caption> 日志文件为 Composer.log </q-item-label>
        </q-item-section>
      </q-item>
    </q-list>
  </q-page>
</template>
