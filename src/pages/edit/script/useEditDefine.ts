import type { QSelectOption } from 'quasar';
import { computed } from 'vue';
import { useI18n } from 'vue-i18n';
import {
  ArchiveTypeEnum,
  ContentTypeEnum,
  DLContentTypeEnum,
  GameDistributionEnum,
  GameTypeEnum,
} from '@/pages/edit/script/define';

export const useEditDefine = () => {
  const { t } = useI18n();

  const archiveTypeOptions = computed((): QSelectOption[] => [
    { label: t('page.edit.archive-info.options.type.none'), value: ArchiveTypeEnum.None },
    { label: t('page.edit.archive-info.options.type.archive'), value: ArchiveTypeEnum.ArchiveFile },
    {
      label: t('page.edit.archive-info.options.type.common-file'),
      value: ArchiveTypeEnum.CommonFile,
    },
    { label: t('page.edit.archive-info.options.type.directory'), value: ArchiveTypeEnum.Directory },
  ]);

  const contentTypeOptions = computed((): QSelectOption[] => [
    { label: t('page.edit.content-info.options.type.undefined'), value: ContentTypeEnum.Undefined },
    { label: t('page.edit.content-info.options.type.game'), value: ContentTypeEnum.Game },
  ]);

  const gameTypeOptions = computed((): QSelectOption[] => [
    {
      label: t('page.edit.content-info.options.game-type.unspecified'),
      value: GameTypeEnum.Unspecified,
    },
    { label: t('page.edit.content-info.options.game-type.rpg'), value: GameTypeEnum.RPG },
    { label: t('page.edit.content-info.options.game-type.slg'), value: GameTypeEnum.SLG },
    { label: t('page.edit.content-info.options.game-type.avg'), value: GameTypeEnum.AVG },
  ]);

  const gameDistributionOptions = computed((): QSelectOption[] => [
    {
      label: t('page.edit.content-info.options.game-dist.unknown'),
      value: GameDistributionEnum.Unknown,
    },
    {
      label: t('page.edit.content-info.options.game-dist.steam'),
      value: GameDistributionEnum.Steam,
    },
    {
      label: t('page.edit.content-info.options.game-dist.dl-site'),
      value: GameDistributionEnum.DLSite,
    },
    {
      label: t('page.edit.content-info.options.game-dist.other'),
      value: GameDistributionEnum.Other,
    },
  ]);

  const dlContentTypeOptions = computed((): QSelectOption[] => [
    {
      label: t('page.edit.content-info.options.game-dl-content.doujin'),
      value: DLContentTypeEnum.Doujin,
    },
    {
      label: t('page.edit.content-info.options.game-dl-content.comics'),
      value: DLContentTypeEnum.Comics,
    },
    {
      label: t('page.edit.content-info.options.game-dl-content.pc-games'),
      value: DLContentTypeEnum.PcGames,
    },
    {
      label: t('page.edit.content-info.options.game-dl-content.smartphone-games'),
      value: DLContentTypeEnum.SmartphoneGames,
    },
    {
      label: t('page.edit.content-info.options.game-dl-content.doujin-r18'),
      value: DLContentTypeEnum.DoujinR18,
    },
    {
      label: t('page.edit.content-info.options.game-dl-content.comics-r18'),
      value: DLContentTypeEnum.ComicsR18,
    },
    {
      label: t('page.edit.content-info.options.game-dl-content.h-games'),
      value: DLContentTypeEnum.HGames,
    },
    {
      label: t('page.edit.content-info.options.game-dl-content.smartphone-games-r18'),
      value: DLContentTypeEnum.SmartphoneGamesR18,
    },
  ]);

  return {
    archiveTypeOptions,
    contentTypeOptions,
    gameTypeOptions,
    gameDistributionOptions,
    dlContentTypeOptions,
  };
};
