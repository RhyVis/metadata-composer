<script lang="ts" setup>
import { useRouteParams } from '@vueuse/router';
import { ref } from 'vue';
import { onBeforeRouteLeave, useRouter } from 'vue-router';
import { useEdit } from '@/pages/edit/script/useEdit.ts';
import EditAlias from '@/pages/edit/comp/EditAlias.vue';
import EditTag from '@/pages/edit/comp/EditTag.vue';
import EditArchiveInfo from '@/pages/edit/comp/EditArchiveInfo.vue';
import EditContentInfo from '@/pages/edit/comp/EditContentInfo.vue';
import EditImage from '@/pages/edit/comp/EditImage.vue';
import { useGlobalStore } from '@/stores/global.ts';
import { storeToRefs } from 'pinia';
import { type QForm, useQuasar } from 'quasar';
import EditTitle from '@/pages/edit/comp/EditTitle.vue';
import EditCollection from '@/pages/edit/comp/EditCollection.vue';
import EditDescription from '@/pages/edit/comp/EditDescription.vue';

const { push } = useRouter();
const { dialog } = useQuasar();
const { isDevMode } = storeToRefs(useGlobalStore());

const extractID = (): string => {
  const param = useRouteParams('id');
  if (!param.value) {
    console.warn('No ID provided, cannot extract ID');
    return 'NEW';
  } else if (typeof param.value !== 'string') {
    console.error(`Invalid ID type, expected a string, but got [${param.value.join(',')}]`);
    return 'NEW';
  }
  return param.value;
};

const formRef = ref<QForm>(null!);

const id = ref(extractID());
const edit = useEdit(id, formRef);
const { editData, isEditMode, pageLock, everEdited, updateData, applyPreset, validate } = edit;

const handleUpdate = async () => {
  if (await updateData()) await push('/');
};

onBeforeRouteLeave((_to, _from, next) => {
  if (pageLock.value) {
    console.warn('Page is locked, cannot leave the edit page.');
    next(false);
  } else if (everEdited.value) {
    dialog({
      title: '确认离开',
      message: '有未保存的更改，确定要离开吗？',
      ok: '离开',
      cancel: '取消',
    })
      .onOk(() => {
        console.info('Leaving the edit page.');
        next();
      })
      .onCancel(() => {
        console.info('Cancelled leaving the edit page.');
        next(false);
      });
  } else {
    next();
  }
});
</script>

<template>
  <q-page padding>
    <q-card>
      <template v-if="isDevMode">
        <q-card-section>
          <div class="text-subtitle2">Develop Info</div>
          <q-separator />
          <div class="text-caption">ID: {{ id }}</div>
          <div class="text-caption">
            EditData:
            <pre>{{ editData }}</pre>
          </div>
          <div class="text-caption">FormRef: {{ formRef }}</div>
          <q-btn label="Test Validation" @click="validate" />
        </q-card-section>
        <q-separator inset />
      </template>

      <q-card-section class="row items-center">
        <div class="text-h6 r-no-sel">
          <template v-if="isEditMode">编辑 {{ editData.title || editData.id }}</template>
          <template v-else>新建元数据</template>
        </div>
        <q-space />
        <q-btn icon="bookmarks" label="预设" outline size="md">
          <q-menu
            anchor="center middle"
            self="top right"
            transition-hide="scale"
            transition-show="scale"
          >
            <q-list style="min-width: 100px">
              <q-item v-close-popup clickable @click="applyPreset('DoujinR18')">
                <q-item-section>DLSite R18 同人</q-item-section>
              </q-item>
            </q-list>
          </q-menu>
        </q-btn>
      </q-card-section>

      <q-separator inset />

      <q-card-section>
        <q-form ref="formRef" @submit.prevent>
          <!-- Title Input -->
          <EditTitle :edit="edit" />
          <!-- Alias Input -->
          <EditAlias :edit="edit" />
          <!-- Tags Input -->
          <EditTag :edit="edit" />
          <!-- Collection Select -->
          <EditCollection :edit="edit" />
          <!-- Description Input -->
          <EditDescription :edit="edit" />
          <!-- Image Input -->
          <EditImage :edit="edit" />
          <!-- Content Info -->
          <EditContentInfo :edit="edit" />
          <!-- Archive Info -->
          <EditArchiveInfo :edit="edit" />
        </q-form>
      </q-card-section>

      <q-separator inset />

      <q-card-actions>
        <q-space />
        <q-btn-group outline>
          <q-btn icon="close" label="退出" outline @click="push('/')" />
          <q-btn :label="isEditMode ? '更新' : '创建'" icon="save" outline @click="handleUpdate" />
        </q-btn-group>
      </q-card-actions>
    </q-card>
  </q-page>
</template>
