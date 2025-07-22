<script lang="ts" setup>
import { useConfigStore } from '@/stores/config.ts';
import { onMounted } from 'vue';
import { useNotify } from '@/composables/useNotify.ts';
import { storeToRefs } from 'pinia';
import { Command } from '@/api/cmd.ts';
import { useConfig } from '@/pages/config/script/useConfig.ts';

const config = useConfigStore();
const { root_data, root_deploy } = storeToRefs(config);
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
          <q-item-label caption>{{ root_data || '`未设置`' }}</q-item-label>
        </q-item-section>
        <q-item-section side top>
          <q-btn flat icon="folder" square @click="handleSelectDir('root_data')" />
        </q-item-section>
      </q-item>

      <q-item v-ripple>
        <q-item-section>
          <q-item-label>部署根</q-item-label>
          <q-item-label caption>{{ root_deploy || '`未设置`' }}</q-item-label>
        </q-item-section>
        <q-item-section side top>
          <q-btn-group flat>
            <q-btn
              v-if="root_deploy"
              flat
              icon="clear"
              square
              @click="handleClearField('root_deploy')"
            />
            <q-btn flat icon="folder" square @click="handleSelectDir('root_deploy')" />
          </q-btn-group>
        </q-item-section>
      </q-item>

      <q-item v-ripple>
        <q-item-section>
          <q-item-label>清除图片缓存</q-item-label>
          <q-item-label caption>清除未使用的图片缓存</q-item-label>
        </q-item-section>
        <q-item-section side top>
          <q-btn flat icon="delete" square @click="handleClearImageCache" />
        </q-item-section>
      </q-item>

      <q-item-label header>数据库</q-item-label>

      <q-item v-ripple>
        <q-item-section>
          <q-item-label>导出元数据</q-item-label>
          <q-item-label caption>导出当前数据库的元数据到 'lib.json'</q-item-label>
        </q-item-section>
        <q-item-section side top>
          <q-btn flat icon="save" square @click="handleExport" />
        </q-item-section>
      </q-item>

      <q-item v-ripple>
        <q-item-section>
          <q-item-label>导入元数据</q-item-label>
          <q-item-label caption>从 'lib.json' 导入元数据到当前数据库</q-item-label>
        </q-item-section>
        <q-item-section side top>
          <q-btn flat icon="upload" square @click="handleImport" />
        </q-item-section>
      </q-item>

      <q-item-label header>目录</q-item-label>

      <q-item v-ripple clickable @click="Command.openConfigDir()">
        <q-item-label>打开配置目录</q-item-label>
        <q-item-label caption> 配置文件为 config.toml </q-item-label>
      </q-item>

      <q-item v-ripple clickable @click="Command.openLogDir()">
        <q-item-label>打开日志目录</q-item-label>
        <q-item-label caption> 日志文件为 composer.log </q-item-label>
      </q-item>
    </q-list>
  </q-page>
</template>
