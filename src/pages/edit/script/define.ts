import type { QSelectOption } from 'quasar';
import type { GameType } from '@/api/types.ts';

export const enum ArchiveTypeEnum {
  None = 'None',
  ArchiveFile = 'ArchiveFile',
  CommonFile = 'CommonFile',
  Directory = 'Directory',
}

export const enum ContentTypeEnum {
  Undefined = 'Undefined',
  Game = 'Game',
}

export const GameTypeEnum = {
  Unspecified: 'Unspecified',
  RPG: 'RPG',
  SLG: 'SLG',
  AVG: 'AVG',
} as const satisfies Record<string, GameType>;

export const enum GameSysPlatformEnum {
  Windows = 'Windows',
  Linux = 'Linux',
  MacOS = 'MacOS',
  Android = 'Android',
  IOS = 'IOS',
  Web = 'Web',
}

export const enum GameDistributionEnum {
  Unknown = 'Unknown',
  Steam = 'Steam',
  DLSite = 'DLSite',
  Other = 'Other',
}

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

export const GameSysPlatformOptions: QSelectOption[] = [
  { label: 'Windows', value: GameSysPlatformEnum.Windows },
  { label: 'Linux', value: GameSysPlatformEnum.Linux },
  { label: 'MacOS', value: GameSysPlatformEnum.MacOS },
  { label: 'Android', value: GameSysPlatformEnum.Android },
  { label: 'iOS', value: GameSysPlatformEnum.IOS },
  { label: 'Web', value: GameSysPlatformEnum.Web },
];

export type EditPreset = 'DoujinR18';
