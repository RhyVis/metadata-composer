import type { QSelectOption } from 'quasar';
import type { GameType } from '@/api/types.ts';
import i18n from '@/i18n';

const { t } = i18n.global;

export const enum ArchiveTypeEnum {
  None = 'None',
  ArchiveFile = 'ArchiveFile',
  CommonFile = 'CommonFile',
  Directory = 'Directory',
}

export const ArchiveTypeOptions: QSelectOption[] = [
  { label: t('page.edit.archive-info.options.type.none'), value: ArchiveTypeEnum.None },
  { label: t('page.edit.archive-info.options.type.archive'), value: ArchiveTypeEnum.ArchiveFile },
  {
    label: t('page.edit.archive-info.options.type.common-file'),
    value: ArchiveTypeEnum.CommonFile,
  },
  { label: t('page.edit.archive-info.options.type.directory'), value: ArchiveTypeEnum.Directory },
];

export const enum ContentTypeEnum {
  Undefined = 'Undefined',
  Game = 'Game',
}

export const ContentTypeOptions: QSelectOption[] = [
  { label: t('page.edit.content-info.options.type.undefined'), value: ContentTypeEnum.Undefined },
  { label: t('page.edit.content-info.options.type.game'), value: ContentTypeEnum.Game },
];

export const GameTypeEnum = {
  Unspecified: 'Unspecified',
  RPG: 'RPG',
  SLG: 'SLG',
  AVG: 'AVG',
} as const satisfies Record<string, GameType>;

export const GameTypeOptions: QSelectOption[] = [
  {
    label: t('page.edit.content-info.options.game-type.unspecified'),
    value: GameTypeEnum.Unspecified,
  },
  { label: t('page.edit.content-info.options.game-type.rpg'), value: GameTypeEnum.RPG },
  { label: t('page.edit.content-info.options.game-type.slg'), value: GameTypeEnum.SLG },
  { label: t('page.edit.content-info.options.game-type.avg'), value: GameTypeEnum.AVG },
];

export const enum GameSysPlatformEnum {
  Windows = 'Windows',
  Linux = 'Linux',
  MacOS = 'MacOS',
  Android = 'Android',
  IOS = 'IOS',
  Web = 'Web',
}

export const GameSysPlatformOptions: QSelectOption[] = [
  { label: 'Windows', value: GameSysPlatformEnum.Windows },
  { label: 'Linux', value: GameSysPlatformEnum.Linux },
  { label: 'MacOS', value: GameSysPlatformEnum.MacOS },
  { label: 'Android', value: GameSysPlatformEnum.Android },
  { label: 'iOS', value: GameSysPlatformEnum.IOS },
  { label: 'Web', value: GameSysPlatformEnum.Web },
];

export const enum GameDistributionEnum {
  Unknown = 'Unknown',
  Steam = 'Steam',
  DLSite = 'DLSite',
  Other = 'Other',
}

export const GameDistributionOptions: QSelectOption[] = [
  {
    label: t('page.edit.content-info.options.game-dist.unknown'),
    value: GameDistributionEnum.Unknown,
  },
  { label: t('page.edit.content-info.options.game-dist.steam'), value: GameDistributionEnum.Steam },
  {
    label: t('page.edit.content-info.options.game-dist.dl-site'),
    value: GameDistributionEnum.DLSite,
  },
  { label: t('page.edit.content-info.options.game-dist.other'), value: GameDistributionEnum.Other },
];

export const enum DLContentTypeEnum {
  Doujin = 'Doujin',
  Comics = 'Comics',
  PcGames = 'PcGames',
  SmartphoneGames = 'SmartphoneGames',
  DoujinR18 = 'DoujinR18',
  ComicsR18 = 'ComicsR18',
  HGames = 'HGames',
  SmartphoneGamesR18 = 'SmartphoneGamesR18',
}

export const DLContentTypeOptions: QSelectOption[] = [
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
];

export type EditPreset = 'DoujinR18';
