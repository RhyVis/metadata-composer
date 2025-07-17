import type { QSelectOption } from 'quasar';

export const enum ArchiveTypeEnum {
  None = 'None',
  ArchiveFile = 'ArchiveFile',
  CommonFile = 'CommonFile',
  Directory = 'Directory',
}

export const ArchiveTypeOptions: QSelectOption[] = [
  { label: 'None', value: ArchiveTypeEnum.None },
  { label: 'Archive File', value: ArchiveTypeEnum.ArchiveFile },
  { label: 'Common File', value: ArchiveTypeEnum.CommonFile },
  { label: 'Directory', value: ArchiveTypeEnum.Directory },
];

export const enum ContentTypeEnum {
  Undefined = 'Undefined',
  Game = 'Game',
}

export const ContentTypeOptions: QSelectOption[] = [
  { label: 'Undefined', value: ContentTypeEnum.Undefined },
  { label: 'Game', value: ContentTypeEnum.Game },
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
  { label: 'IOS', value: GameSysPlatformEnum.IOS },
  { label: 'Web', value: GameSysPlatformEnum.Web },
];

export const enum GameDistributionEnum {
  Unknown = 'Unknown',
  Steam = 'Steam',
  DLSite = 'DLSite',
}

export const GameDistributionOptions: QSelectOption[] = [
  { label: 'Unknown', value: GameDistributionEnum.Unknown },
  { label: 'Steam', value: GameDistributionEnum.Steam },
  { label: 'DLSite', value: GameDistributionEnum.DLSite },
];

export const enum DLContentTypeEnum {
  Maniax = 'Maniax',
  Books = 'Books',
  Pro = 'Pro',
  AppX = 'AppX',
}

export const DLContentTypeOptions: QSelectOption[] = [
  { label: 'Maniax', value: DLContentTypeEnum.Maniax },
  { label: 'Books', value: DLContentTypeEnum.Books },
  { label: 'Pro', value: DLContentTypeEnum.Pro },
  { label: 'AppX', value: DLContentTypeEnum.AppX },
];
