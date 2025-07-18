<script lang="ts" setup>
import { useRouter } from 'vue-router';
import { MainColDef, PaginationOptions, RowPageSizes } from '@/pages/main/script/define.ts';
import { useTable } from '@/pages/main/script/useTable.ts';
import { storeToRefs } from 'pinia';
import { useTableStore } from '@/stores/table.ts';
import type { Metadata } from '@/api/types.ts';
import AsyncImage from '@/components/AsyncImage.vue';
import { computed, onMounted } from 'vue';
import { useQuasar } from 'quasar';
import { useLibraryStore } from '@/stores/library.ts';
import { Command } from '@/api/cmd.ts';
import { useNotify } from '@/composables/useNotify.ts';
import { isDeployable, isDeployed } from '@/pages/main/script/function.ts';
import { selectDirectory } from '@/api/dialog.ts';
import { useConfigStore } from '@/stores/config.ts';
import { formatBytes } from '@/api/util.ts';

const { push } = useRouter();
const { notifySuccess, notifyError } = useNotify();
const { dark, loading } = useQuasar();
const { searchTag, searchByRegex, rows } = useTable();

const config = useConfigStore();
const library = useLibraryStore();
const tableStore = useTableStore();
const { fetch } = library;
const { pagination, visibleColumns } = storeToRefs(useTableStore());
const { totalFileSize, size } = storeToRefs(library);
const { root_deploy } = storeToRefs(useConfigStore());

const textClazz = computed(() => (dark.isActive ? 'text-grey-5' : 'text-grey-9'));

const handleReload = async () => {
  console.info('Reloading table data...');
  loading.show();
  try {
    await fetch();
    notifySuccess('数据已刷新');
  } catch (e) {
    console.error(e);
    notifyError('数据刷新失败', e);
  } finally {
    loading.hide();
  }
};
const handleEdit = (id: string) => {
  console.info(`Editing item with id: ${id}`);
  push(`/edit/${id}`);
};
const handleRemove = async (id: string) => {
  console.info(`Removing item with id: ${id}`);
  loading.show();
  try {
    await Command.metadataDelete(id);
    await fetch();
    notifySuccess(`已成功删除 '${id}'`);
  } catch (e) {
    console.error(e);
    notifyError(`删除 '${id}' 失败`, e);
  } finally {
    loading.hide();
  }
};
const handleDeploy = async (id: string, useDeployDir: boolean) => {
  console.info(`Deploying item with id: ${id}`);
  if (useDeployDir) {
    loading.show({
      message: `正在部署 '${id}'...`,
    });
    try {
      await Command.metadataDeploy(id, {
        use_config_dir: true,
        target_dir: null,
      });
      await fetch();
    } catch (e) {
      console.error(e);
      notifyError(`部署 '${id}' 失败`, e);
    } finally {
      loading.hide();
    }
  } else {
    try {
      const path = await selectDirectory();
      if (path) {
        loading.show({
          message: `正在部署 '${id}' 到 ${path}...`,
        });
        try {
          await Command.metadataDeploy(id, {
            use_config_dir: false,
            target_dir: path,
          });
          await fetch();
          notifySuccess(`已成功部署 '${id}' 到 ${path}`);
        } catch (e) {
          console.error(e);
          notifyError(`部署 '${id}' 失败`, e);
        } finally {
          loading.hide();
        }
      } else {
        notifyError('部署取消', '未选择有效的目录');
      }
    } catch (e) {
      console.error(e);
      notifyError('选择目录失败', e);
    }
  }
};
const handleDeployOff = async (id: string) => {
  console.info(`Un-deploying item with id: ${id}`);
  loading.show({
    message: `正在取消部署 '${id}'...`,
  });
  try {
    await Command.metadataDeployOff(id);
    await fetch();
    notifySuccess(`已成功取消部署 '${id}'`);
  } catch (e) {
    console.error(e);
    notifyError(`取消部署 '${id}' 失败`, e);
  } finally {
    loading.hide();
  }
};

onMounted(() =>
  tableStore.$tauri.start().catch((e) => {
    console.error(`Failed to start table store: ${e}`);
    notifyError('表格加载失败', e);
  }),
);
</script>

<template>
  <q-page padding>
    <div class="text-center">
      <q-table
        v-model:pagination="pagination"
        :columns="MainColDef"
        :rows="rows"
        :rows-per-page-options="RowPageSizes"
        :visible-columns="visibleColumns"
        flat
        grid
        row-key="id"
      >
        <template #top-left>
          <q-btn-group outline>
            <q-btn outline @click="handleReload">
              <div class="row items-center text-subtitle2">
                {{ formatBytes(totalFileSize) }} | {{ size }}
              </div>
            </q-btn>
            <q-btn icon="add" label="新建" outline square @click="handleEdit('new')" />
          </q-btn-group>
        </template>
        <template #top-right>
          <div class="q-gutter-sm row items-center">
            <q-input v-model="searchTag" dense outlined placeholder="搜索">
              <template #append>
                <q-icon name="delete" v-if="searchTag" @click="searchTag = ''" />
                <q-icon name="search" v-else />
                <q-checkbox
                  v-model="searchByRegex"
                  checked-icon="fa-solid fa-code"
                  unchecked-icon="fa-solid fa-font"
                >
                  <q-tooltip class="r-no-sel">{{
                    searchByRegex ? '正则表达式搜索' : '普通搜索'
                  }}</q-tooltip>
                </q-checkbox>
              </template>
            </q-input>
            <q-select
              v-model="pagination.sortBy"
              :options="PaginationOptions"
              dense
              display-value="排序"
              emit-value
              map-options
              options-dense
              outlined
            >
              <template #after-options>
                <div class="row items-center r-no-sel q-px-md">
                  <div class="q-mr-xs">{{ pagination.descending ? '降序' : '升序' }}</div>
                  <q-checkbox
                    v-model="pagination.descending"
                    checked-icon="fa-solid fa-sort-down"
                    color="primary"
                    keep-color
                    size="sm"
                    unchecked-icon="fa-solid fa-sort-up"
                  />
                </div>
              </template>
            </q-select>
            <q-select
              v-model="visibleColumns"
              :options="MainColDef.filter((col) => col.name != 'title')"
              dense
              display-value="显示内容"
              emit-value
              map-options
              multiple
              option-value="name"
              options-dense
              outlined
            />
          </div>
        </template>

        <template #item="{ cols, row }">
          <div class="q-pa-xs col-xs-12 col-sm-6 col-md-4 flex flex-col">
            <q-card class="full-height full-width column" bordered flat>
              <q-list class="q-my-sm text-left" dense>
                <q-item v-for="col in cols" :key="`col-${col.name}`">
                  <q-item-section>
                    <!-- Description -->
                    <template v-if="col.name === 'description'">
                      <div class="r-no-sel text-weight-medium">{{ col.label }}</div>
                      <div
                        class="text-body2 q-ml-xs"
                        v-for="(line, lineIdx) in col.value"
                        :class="textClazz"
                        :key="lineIdx"
                      >
                        {{ line }}
                      </div>
                    </template>
                    <!-- Image -->
                    <template v-else-if="col.name === 'image'">
                      <div class="r-no-sel text-weight-medium">{{ col.label }}</div>
                      <div class="full-width" style="max-height: 260px">
                        <AsyncImage :hash="col.value">
                          <template #state="{ assetUrl, error, loading }">
                            <q-card
                              class="full-width flex items-center justify-center"
                              v-if="!col.value"
                              flat
                              style="height: 256px"
                            >
                              <div class="text-center column items-center">
                                <q-icon name="image" color="grey-6" size="4rem" />
                                <div class="text-subtitle2 text-grey-8 q-mt-sm">无图片</div>
                              </div>
                            </q-card>
                            <q-spinner v-else-if="loading" />
                            <q-banner class="bg-negative text-white" v-else-if="error">
                              {{ error }}
                            </q-banner>
                            <q-img
                              class="q-ml-xs"
                              v-else
                              :src="assetUrl"
                              contain
                              fit="scale-down"
                              style="height: 256px"
                            />
                          </template>
                        </AsyncImage>
                      </div>
                    </template>
                    <!-- Common Fields -->
                    <template v-else>
                      <div class="r-no-sel text-weight-medium">{{ col.label }}</div>
                      <div class="text-body2 q-ml-xs" :class="textClazz">{{ col.value }}</div>
                    </template>
                  </q-item-section>
                </q-item>
              </q-list>
              <q-space />
              <q-separator inset />
              <q-card-actions class="q-mt-auto" align="right">
                <q-btn-group flat>
                  <template v-if="isDeployable(row as Metadata)">
                    <q-btn
                      v-if="config.hasDeployRoot"
                      color="primary"
                      flat
                      icon="create_new_folder"
                      size="sm"
                    >
                      <q-tooltip>部署到指定目录</q-tooltip>
                      <q-popup-proxy>
                        <q-card>
                          <q-card-section>
                            <div class="r-no-sel text-subtitle2">
                              <div>部署到设置目录或自定义目录 -></div>
                              <div>当前设置目录: {{ root_deploy }}</div>
                            </div>
                          </q-card-section>
                          <q-separator />
                          <q-card-actions align="right">
                            <q-btn-group flat>
                              <q-btn
                                v-close-popup
                                flat
                                label="设置目录"
                                size="sm"
                                @click="handleDeploy(row.id, true)"
                              />
                              <q-btn
                                v-close-popup
                                flat
                                label="自定义目录"
                                size="sm"
                                @click="handleDeploy(row.id, false)"
                              />
                            </q-btn-group>
                          </q-card-actions>
                        </q-card>
                      </q-popup-proxy>
                    </q-btn>
                    <q-btn
                      v-else
                      color="primary"
                      flat
                      icon="create_new_folder"
                      size="sm"
                      @click="handleDeploy(row.id, false)"
                    >
                      <q-tooltip>部署到指定目录</q-tooltip>
                    </q-btn>
                  </template>
                  <q-btn
                    v-if="isDeployed(row as Metadata)"
                    color="primary"
                    flat
                    icon="folder_off"
                    size="sm"
                  >
                    <q-tooltip>取消部署</q-tooltip>
                    <q-popup-proxy>
                      <q-card>
                        <q-card-section>
                          <div class="r-no-sel text-subtitle2">确定要取消部署吗</div>
                        </q-card-section>
                        <q-separator />
                        <q-card-actions align="right">
                          <q-btn-group flat>
                            <q-btn v-close-popup flat icon="close" size="sm" />
                            <q-btn
                              v-close-popup
                              flat
                              icon="check"
                              size="sm"
                              @click="handleDeployOff(row.id)"
                            />
                          </q-btn-group>
                        </q-card-actions>
                      </q-card>
                    </q-popup-proxy>
                  </q-btn>
                  <q-btn flat icon="edit" size="sm" @click="handleEdit((row as Metadata).id)">
                    <q-tooltip> 编辑条目 </q-tooltip>
                  </q-btn>
                  <q-btn color="negative" flat icon="delete" size="sm">
                    <q-tooltip> 删除条目 </q-tooltip>
                    <q-popup-proxy>
                      <q-card>
                        <q-card-section>
                          <div class="r-no-sel text-subtitle2">
                            确定要删除'{{ (row as Metadata).title }}'吗
                          </div>
                        </q-card-section>
                        <q-separator />
                        <q-card-actions align="right">
                          <q-btn-group flat>
                            <q-btn v-close-popup flat icon="close" size="sm" />
                            <q-btn
                              v-close-popup
                              flat
                              icon="check"
                              size="sm"
                              @click="handleRemove((row as Metadata).id)"
                            />
                          </q-btn-group>
                        </q-card-actions>
                      </q-card>
                    </q-popup-proxy>
                  </q-btn>
                </q-btn-group>
              </q-card-actions>
            </q-card>
          </div>
        </template>
      </q-table>
    </div>
  </q-page>
</template>
