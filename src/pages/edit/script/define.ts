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
