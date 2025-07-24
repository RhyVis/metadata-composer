<script lang="ts" setup>
import type { Metadata } from '@/api/types';
import { getDeployPath, isDeployable, isDeployed } from '@/pages/main/script/function';
import type { UseOperation } from '@/pages/main/script/useOperation';
import { useConfigStore } from '@/stores/config';
import { openPath } from '@tauri-apps/plugin-opener';
import { storeToRefs } from 'pinia';
import { computed } from 'vue';

interface ColItem {
  name: string;
  label: string;
  value: unknown;
}

const { cols, row, operation } = defineProps<{
  cols: ColItem[];
  row: Metadata;
  operation: UseOperation;
}>();
const emit = defineEmits<{
  edit: [id: string];
}>();

const { handleDeploy, handleDeployOff, handleRemove } = operation;
const { isDarkMode, pathDeploy } = storeToRefs(useConfigStore());

const innerTextClazz = computed(() => (isDarkMode.value ? 'text-grey-5' : 'text-grey-9'));
</script>

<template>
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
                        <div class="text-subtitle2 text-grey-8 q-mt-sm">
                          {{ $t('page.main.no-image') }}
                        </div>
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
            <q-btn v-if="pathDeploy" flat icon="create_new_folder">
              <q-tooltip>{{ $t('page.main.deploy.tooltip.config-or-custom') }}</q-tooltip>
              <q-popup-proxy>
                <q-card>
                  <q-card-section>
                    <div class="r-no-sel text-subtitle2">
                      <div>{{ `${$t('page.main.deploy.tooltip.config-or-custom')} ->` }}</div>
                      <div>
                        {{ $t('page.main.deploy.tooltip.current-config-dir', [pathDeploy]) }}
                      </div>
                    </div>
                  </q-card-section>
                  <q-separator />
                  <q-card-actions align="right">
                    <q-btn-group flat>
                      <q-btn
                        v-close-popup
                        :label="$t('page.main.deploy.config-dir')"
                        flat
                        size="sm"
                        @click="handleDeploy(row.id, true)"
                      >
                        <template #loading></template>
                      </q-btn>
                      <q-btn
                        v-close-popup
                        :label="$t('page.main.deploy.custom-dir')"
                        flat
                        size="sm"
                        @click="handleDeploy(row.id, false)"
                      />
                    </q-btn-group>
                  </q-card-actions>
                </q-card>
              </q-popup-proxy>
            </q-btn>
            <q-btn v-else flat icon="create_new_folder" @click="handleDeploy(row.id, false)">
              <q-tooltip>{{ $t('page.main.deploy.tooltip.to-target') }}</q-tooltip>
            </q-btn>
          </template>
          <template v-if="isDeployed(row)">
            <q-btn
              v-if="getDeployPath(row)"
              flat
              icon="folder"
              @click="openPath(getDeployPath(row))"
            >
              <q-tooltip>{{ $t('page.main.deploy.open-dir') }}</q-tooltip>
            </q-btn>
            <q-btn flat icon="folder_off">
              <q-tooltip>{{ $t('page.main.deploy.deploy-off') }}</q-tooltip>
              <q-popup-proxy>
                <q-card>
                  <q-card-section>
                    <div class="r-no-sel text-subtitle2">
                      {{ $t('page.main.deploy.confirm-deploy-off') }}
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
                        @click="handleDeployOff(row.id)"
                      />
                    </q-btn-group>
                  </q-card-actions>
                </q-card>
              </q-popup-proxy>
            </q-btn>
          </template>

          <!-- Action Edit -->
          <q-btn flat icon="edit" @click="emit('edit', row.id)">
            <q-tooltip> {{ $t('page.main.edit.tooltip') }} </q-tooltip>
          </q-btn>

          <!-- Action Delete -->
          <q-btn color="negative" flat icon="delete">
            <q-tooltip> {{ $t('page.main.delete.tooltip') }} </q-tooltip>
            <q-popup-proxy>
              <q-card>
                <q-card-section>
                  <div class="r-no-sel text-subtitle2">
                    {{ $t('page.main.delete.confirm', [row.title || row.id]) }}
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
                      @click="handleRemove(row.id)"
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
