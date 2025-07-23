<script lang="ts" setup>
import { useRouter } from 'vue-router';
import {
  FilterOptions,
  MainColDef,
  PaginationOptions,
  RowPageSizes,
} from '@/pages/main/script/define.ts';
import { useTable } from '@/pages/main/script/useTable.ts';
import { storeToRefs } from 'pinia';
import { useTableStore } from '@/stores/table.ts';
import type { Metadata } from '@/api/types.ts';
import AsyncImage from '@/components/AsyncImage.vue';
import { computed, onMounted } from 'vue';
import { useQuasar } from 'quasar';
import { useLibraryStore } from '@/stores/library.ts';
import { useNotify } from '@/composables/useNotify.ts';
import { getDeployPath, isDeployable, isDeployed } from '@/pages/main/script/function.ts';
import { useConfigStore } from '@/stores/config.ts';
import { formatBytes } from '@/api/util.ts';
import { useOperation } from '@/pages/main/script/useOperation.ts';
import { openPath } from '@tauri-apps/plugin-opener';

const { push } = useRouter();
const { notifyError } = useNotify();
const { dark } = useQuasar();

const tableStore = useTableStore();
const { pagination, visibleColumns } = storeToRefs(tableStore);
const { totalFileSize, size } = storeToRefs(useLibraryStore());
const { path_deploy, hasDeployPath } = storeToRefs(useConfigStore());

const { filterType, searchTag, searchByRegex, rows } = useTable();
const { handleReload, handleRemove, handleDeploy, handleDeployOff } = useOperation();

const innerTextClazz = computed(() => (dark.isActive ? 'text-grey-5' : 'text-grey-9'));

const handleEdit = (id: string) => {
  console.info(`Editing item with id: ${id}`);
  push(`/edit/${id}`);
};

onMounted(() =>
  tableStore.$tauri.start().then(
    () => tableStore.syncDeploymentCache(),
    (e) => {
      console.error(`Failed to start table store: ${e}`);
      notifyError('表格加载失败', e);
    },
  ),
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
            <q-btn outline>
              <div class="row items-center text-subtitle2">{{ formatBytes(totalFileSize) }}</div>
              <q-menu anchor="top middle" self="top middle">
                <q-list separator>
                  <q-item v-close-popup clickable @click="handleReload">
                    <q-item-section avatar>
                      <q-icon name="refresh" />
                    </q-item-section>
                    <q-item-section>刷新</q-item-section>
                  </q-item>
                  <q-item v-close-popup clickable @click="handleEdit('new')">
                    <q-item-section avatar>
                      <q-icon name="add" />
                    </q-item-section>
                    <q-item-section>新建</q-item-section>
                  </q-item>
                </q-list>
              </q-menu>
            </q-btn>
            <q-separator vertical />
            <q-btn class="text-subtitle2 r-no-sel" outline>
              {{ size === (rows as []).length ? `${size}` : `${(rows as []).length} / ${size}` }}
            </q-btn>
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
              v-model="filterType"
              :options="FilterOptions"
              dense
              display-value="过滤"
              emit-value
              map-options
              options-dense
              outlined
            />
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

        <!-- Item Template -->
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
                        :class="innerTextClazz"
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
                      <div class="text-body2 q-ml-xs" :class="innerTextClazz">{{ col.value }}</div>
                    </template>
                  </q-item-section>
                </q-item>
              </q-list>
              <q-space />
              <q-separator inset />

              <!-- Actions -->
              <q-card-actions class="q-mt-auto" align="right">
                <q-btn-group flat>
                  <!-- Action Deploy -->
                  <template v-if="isDeployable(row as Metadata)">
                    <q-btn v-if="hasDeployPath" flat icon="create_new_folder">
                      <q-tooltip>部署到设置目录或自定义目录</q-tooltip>
                      <q-popup-proxy>
                        <q-card>
                          <q-card-section>
                            <div class="r-no-sel text-subtitle2">
                              <div>部署到设置目录或自定义目录 -></div>
                              <div>当前设置目录: '{{ path_deploy }}'</div>
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
                      flat
                      icon="create_new_folder"
                      @click="handleDeploy(row.id, false)"
                    >
                      <q-tooltip>部署到指定目录</q-tooltip>
                    </q-btn>
                  </template>
                  <template v-if="isDeployed(row as Metadata)">
                    <q-btn
                      v-if="getDeployPath(row as Metadata)"
                      flat
                      icon="folder"
                      @click="openPath(getDeployPath(row as Metadata))"
                    >
                      <q-tooltip>打开部署文件夹</q-tooltip>
                    </q-btn>
                    <q-btn flat icon="folder_off">
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
                  </template>

                  <!-- Action Edit -->
                  <q-btn flat icon="edit" @click="handleEdit((row as Metadata).id)">
                    <q-tooltip> 编辑条目 </q-tooltip>
                  </q-btn>

                  <!-- Action Delete -->
                  <q-btn color="negative" flat icon="delete">
                    <q-tooltip> 删除条目 </q-tooltip>
                    <q-popup-proxy>
                      <q-card>
                        <q-card-section>
                          <div class="r-no-sel text-subtitle2">
                            确定要删除'{{ (row as Metadata).title || (row as Metadata).id }}'吗
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
