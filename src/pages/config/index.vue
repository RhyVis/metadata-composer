<script lang="ts" setup>
import { useConfigStore } from '@/stores/config.ts';
import { onMounted } from 'vue';
import { useNotify } from '@/composables/useNotify.ts';
import { storeToRefs } from 'pinia';
import { selectDirectory } from '@/api/dialog.ts';
import type { InternalConfig } from '@/api/types.ts';
import { Command } from '@/api/cmd.ts';

const config = useConfigStore();
const { root_data, root_deploy } = storeToRefs(config);
const { notifyError } = useNotify();

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

onMounted(() =>
  config.sync().then(
    () => console.info('Successfully synced config file'),
    (e) => notifyError('配置同步失败', e),
  ),
);
</script>

<template>
  <q-page>
    <q-list bordered padding>
      <q-item-label header>内部设置</q-item-label>

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
    </q-list>
  </q-page>
</template>
