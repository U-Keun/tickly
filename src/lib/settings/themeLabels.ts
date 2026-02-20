import type { ThemeColors } from '../../types';

export type ThemeColorKey = keyof ThemeColors;

const themeColorKeyMap: Record<ThemeColorKey, string> = {
  paper: 'colorPaper',
  canvas: 'colorCanvas',
  mist: 'colorMist',
  stroke: 'colorStroke',
  ink: 'colorInk',
  inkMuted: 'colorInkMuted',
  accentSky: 'colorAccentSky',
  accentSkyStrong: 'colorAccentSkyStrong',
  accentMint: 'colorAccentMint',
  accentMintStrong: 'colorAccentMintStrong',
  accentPeach: 'colorAccentPeach',
  accentPeachStrong: 'colorAccentPeachStrong',
  white: 'colorWhite',
  border: 'colorBorder'
};

const themePresetNameMap: Record<string, string> = {
  default: 'themeDefault',
  dark: 'themeDark',
  ocean: 'themeOcean',
  forest: 'themeForest',
  sunset: 'themeSunset'
};

export const themeColorKeys = Object.keys(themeColorKeyMap) as ThemeColorKey[];

export function getThemePresetName(
  presetId: string,
  translate: (key: string) => string
): string {
  const key = themePresetNameMap[presetId];
  return key ? translate(key) : presetId;
}

export function getThemeColorLabel(
  colorKey: ThemeColorKey,
  translate: (key: string) => string
): string {
  const key = themeColorKeyMap[colorKey];
  return key ? translate(key) : colorKey;
}
