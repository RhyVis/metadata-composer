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
import EditRating from '@/pages/edit/comp/EditRating.vue';
import { storeToRefs } from 'pinia';
import { type QForm, useQuasar } from 'quasar';
import EditTitle from '@/pages/edit/comp/EditTitle.vue';
import EditCollection from '@/pages/edit/comp/EditCollection.vue';
import EditDescription from '@/pages/edit/comp/EditDescription.vue';
import { useConfigStore } from '@/stores/config';
import { useI18n } from 'vue-i18n';

const { t } = useI18n();
const { push } = useRouter();
const { dialog } = useQuasar();
const { isDevMode } = storeToRefs(useConfigStore());

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
      title: t('page.edit.dialog.exit-guard.title'),
      message: t('page.edit.dialog.exit-guard.message'),
      ok: t('general.ok'),
      cancel: t('general.cancel'),
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
          <div class="text-subtitle2">{{ $t('develop.develop-info') }}</div>
          <q-separator />
          <div class="text-caption">{{ $t('develop.display.id', [id]) }}</div>
          <div class="text-caption">
            EditData:
            <pre>{{ editData }}</pre>
          </div>
          <q-btn :label="$t('develop.display.test-validation')" @click="validate" />
        </q-card-section>
        <q-separator inset />
      </template>

      <q-card-section class="row items-center">
        <div class="text-h6 r-no-sel">
          <template v-if="isEditMode">{{
            $t('page.edit.title.edit', [editData.title || editData.id])
          }}</template>
          <template v-else>{{ $t('page.edit.title.create') }}</template>
        </div>
        <q-space />
        <q-btn :label="$t('page.edit.preset.header')" icon="bookmarks" outline size="md">
          <q-menu
            anchor="center middle"
            self="top right"
            transition-hide="scale"
            transition-show="scale"
          >
            <q-list style="min-width: 100px">
              <q-item v-close-popup clickable @click="applyPreset('DoujinR18')">
                <q-item-section>{{ $t('page.edit.preset.doujin-r18') }}</q-item-section>
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
          <!-- Rating Input -->
          <EditRating :edit="edit" />
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
          <q-btn :label="$t('general.exit')" icon="close" outline @click="push('/')" />
          <q-btn
            :label="isEditMode ? $t('general.update') : $t('general.create')"
            icon="save"
            outline
            @click="handleUpdate"
          />
        </q-btn-group>
      </q-card-actions>
    </q-card>
  </q-page>
</template>
