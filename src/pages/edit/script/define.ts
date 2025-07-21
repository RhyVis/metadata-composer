import type { QSelectOption } from 'quasar';
import type { GameType } from '@/api/types.ts';

export const enum ArchiveTypeEnum {
  None = 'None',
  ArchiveFile = 'ArchiveFile',
  CommonFile = 'CommonFile',
  Directory = 'Directory',
}

export const ArchiveTypeOptions: QSelectOption[] = [
  { label: '无', value: ArchiveTypeEnum.None },
  { label: '压缩档', value: ArchiveTypeEnum.ArchiveFile },
  { label: '文件', value: ArchiveTypeEnum.CommonFile },
  { label: '目录', value: ArchiveTypeEnum.Directory },
];

export const enum ContentTypeEnum {
  Undefined = 'Undefined',
  Game = 'Game',
}

export const ContentTypeOptions: QSelectOption[] = [
  { label: '未知', value: ContentTypeEnum.Undefined },
  { label: '游戏', value: ContentTypeEnum.Game },
];

export const GameTypeEnum = {
  Unspecified: 'Unspecified',
  RPG: 'RPG',
  SLG: 'SLG',
  AVG: 'AVG',
} as const satisfies Record<string, GameType>;

export const GameTypeOptions: QSelectOption[] = [
  { label: '未指定', value: GameTypeEnum.Unspecified },
  { label: '角色扮演游戏 (RPG)', value: GameTypeEnum.RPG },
  { label: '策略游戏 (SLG)', value: GameTypeEnum.SLG },
  { label: '冒险游戏 (AVG)', value: GameTypeEnum.AVG },
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
  { label: '未知', value: GameDistributionEnum.Unknown },
  { label: 'Steam', value: GameDistributionEnum.Steam },
  { label: 'DLSite', value: GameDistributionEnum.DLSite },
  { label: '其他', value: GameDistributionEnum.Other },
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
  { label: '同人', value: DLContentTypeEnum.Doujin },
  { label: '漫画', value: DLContentTypeEnum.Comics },
  { label: 'PC软件', value: DLContentTypeEnum.PcGames },
  { label: '手机游戏', value: DLContentTypeEnum.SmartphoneGames },
  { label: '同人 R18', value: DLContentTypeEnum.DoujinR18 },
  { label: '漫画 R18', value: DLContentTypeEnum.ComicsR18 },
  { label: '美少女游戏 R18', value: DLContentTypeEnum.HGames },
  { label: '手机游戏 R18', value: DLContentTypeEnum.SmartphoneGamesR18 },
];

export type EditPreset = 'DoujinR18';
