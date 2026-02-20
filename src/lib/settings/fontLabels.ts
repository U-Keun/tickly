import type { FontSize } from '../../types';

const fontPresetNameMap: Record<string, string> = {
  system: 'fontSystem',
  'noto-sans': 'fontNotoSans',
  pretendard: 'fontPretendard',
  monospace: 'fontMonospace'
};

const fontSizeNameMap: Record<FontSize, string> = {
  small: 'fontSizeSmall',
  medium: 'fontSizeMedium',
  large: 'fontSizeLarge'
};

function getSystemFontName(): string {
  const platform = navigator.platform.toLowerCase();
  if (platform.includes('mac') || platform.includes('iphone') || platform.includes('ipad')) {
    return 'San Francisco';
  }
  if (platform.includes('win')) {
    return 'Segoe UI';
  }
  if (/android/i.test(navigator.userAgent)) {
    return 'Roboto';
  }
  return 'System';
}

export function getFontPresetName(
  presetId: string,
  translate: (key: string) => string
): string {
  if (presetId === 'system') {
    return `${translate('fontSystem')} (${getSystemFontName()})`;
  }

  const key = fontPresetNameMap[presetId];
  return key ? translate(key) : presetId;
}

export function getFontSizeName(
  size: FontSize,
  translate: (key: string) => string
): string {
  const key = fontSizeNameMap[size];
  return key ? translate(key) : size;
}
