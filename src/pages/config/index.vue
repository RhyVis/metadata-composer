<script lang="ts" setup>
import { useConfigStore } from '@/stores/config.ts';
import { onMounted } from 'vue';
import { useNotify } from '@/composables/useNotify.ts';
import { storeToRefs } from 'pinia';
import { selectDirectory } from '@/api/dialog.ts';
import type { InternalConfig } from '@/api/types.ts';
import { Command } from '@/api/cmd.ts';
import { useQuasar } from 'quasar';

const { loading } = useQuasar();

const config = useConfigStore();
const { root_data, root_deploy } = storeToRefs(config);
const { notifyError, notifySuccess } = useNotify();

const handleSelectDir = async (name: keyof InternalConfig) => {
  try {
    const path = await selectDirectory();
    if (!path) {
      notifyError('选择路径失败', '未选择任何目录');
      return;
    }
    switch (name) {
      case 'root_data': {
        await Command.configUpdate('root_data', path);
        await config.sync();
        break;
      }
      case 'root_deploy': {
        await Command.configUpdate('root_deploy', path);
        await config.sync();
        break;
      }
    }
  } catch (e) {
    console.error(e);
  }
};
const handleClearRootDeploy = async () => {
  try {
    await Command.configUpdate('root_deploy', undefined);
    await config.sync();
  } catch (e) {
    console.error(e);
    notifyError('清除部署根失败', e);
  }
};
const handleClearImageCache = async () => {
  try {
    loading.show({ message: '正在清除图片缓存...' });
    const count = await Command.utilClearUnusedImages();
    if (count > 0) {
      notifySuccess('清除图片缓存成功', `已清除 ${count} 张未使用的图片`);
    } else {
      notifySuccess('清除图片缓存成功', '没有未使用的图片');
    }
  } catch (e) {
    console.error(e);
    notifyError('清除图片缓存失败', e);
  } finally {
    loading.hide();
  }
};
const handleExport = async () => {
  try {
    loading.show({ message: '正在导出元数据...' });
    await Command.metadataExport();
    notifySuccess('导出元数据成功');
  } catch (e) {
    console.error(e);
    notifyError('导出元数据失败', e);
  } finally {
    loading.hide();
  }
};
const handleImport = async () => {
  try {
    loading.show({ message: '正在导入元数据...' });
    await Command.metadataImport();
    notifySuccess('导入元数据成功', '刷新数据库以查看更改');
  } catch (e) {
    console.error(e);
    notifyError('导入元数据失败', e);
  } finally {
    loading.hide();
  }
};

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
          <q-item-label caption>{{ root_data || '`unset`' }}</q-item-label>
        </q-item-section>
        <q-item-section side top>
          <q-btn flat icon="folder" square @click="handleSelectDir('root_data')" />
        </q-item-section>
      </q-item>

      <q-item v-ripple>
        <q-item-section>
          <q-item-label>部署根</q-item-label>
          <q-item-label caption>{{ root_deploy || '`unset`' }}</q-item-label>
        </q-item-section>
        <q-item-section side top>
          <q-btn-group flat>
            <q-btn v-if="root_deploy" flat icon="clear" square @click="handleClearRootDeploy" />
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
    </q-list>
  </q-page>
</template>
