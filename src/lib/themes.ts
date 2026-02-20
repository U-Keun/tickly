import type { ThemeColors, ThemePreset } from '../types';
import { getSetting, setSetting } from './api/settingsApi';
import * as widgetApi from './api/widgetApi';

// Default theme (기본)
const defaultColors: ThemeColors = {
  paper: '#f8f7f3',
  canvas: '#f2efe8',
  mist: '#ede9e1',
  stroke: '#e2ded5',
  ink: '#5b5852',
  inkMuted: '#7a776f',
  accentSky: '#a8bddb',
  accentSkyStrong: '#8ea9cf',
  accentMint: '#bfd9c8',
  accentMintStrong: '#a7c8b5',
  accentPeach: '#e9c1ad',
  accentPeachStrong: '#dba892',
  white: '#ffffff',
  border: '#e5e7eb',
};

// Dark theme (다크)
const darkColors: ThemeColors = {
  paper: '#1f2937',
  canvas: '#111827',
  mist: '#374151',
  stroke: '#4b5563',
  ink: '#f3f4f6',
  inkMuted: '#9ca3af',
  accentSky: '#0042a9',
  accentSkyStrong: '#3a87fe',
  accentMint: '#ffffff',
  accentMintStrong: '#c2c2c2',
  accentPeach: '#5c5c5c',
  accentPeachStrong: '#333333',
  white: '#1f2937',
  border: '#374151',
};

// Ocean theme (오션)
const oceanColors: ThemeColors = {
  paper: '#e0f2fe',
  canvas: '#bae6fd',
  mist: '#7dd3fc',
  stroke: '#38bdf8',
  ink: '#0c4a6e',
  inkMuted: '#075985',
  accentSky: '#0284c7',
  accentSkyStrong: '#0369a1',
  accentMint: '#93e3fd',
  accentMintStrong: '#00a1d8',
  accentPeach: '#ffd582',
  accentPeachStrong: '#fee4ad',
  white: '#f0f9ff',
  border: '#7dd3fc',
};

// Forest theme (포레스트)
const forestColors: ThemeColors = {
  paper: '#ecfdf5',
  canvas: '#cde8b5',
  mist: '#b1dd8b',
  stroke: '#38571a',
  ink: '#263e0f',
  inkMuted: '#4e7a27',
  accentSky: '#6f760a',
  accentSkyStrong: '#4f5504',
  accentMint: '#76bb40',
  accentMintStrong: '#96d35f',
  accentPeach: '#a6cd00',
  accentPeachStrong: '#70ca00',
  white: '#f0fdf4',
  border: '#dfeed4',
};

// Sunset theme (선셋)
const sunsetColors: ThemeColors = {
  paper: '#fef3c7',
  canvas: '#fde68a',
  mist: '#fcd34d',
  stroke: '#fbbf24',
  ink: '#78350f',
  inkMuted: '#92400e',
  accentSky: '#ff9300',
  accentSkyStrong: '#ff6a00',
  accentMint: '#fb923c',
  accentMintStrong: '#f97316',
  accentPeach: '#ffc777',
  accentPeachStrong: '#feb43f',
  white: '#fffbeb',
  border: '#fcd34d',
};

export const themePresets: ThemePreset[] = [
  { id: 'default', name: '기본', colors: defaultColors },
  { id: 'dark', name: '다크', colors: darkColors },
  { id: 'ocean', name: '오션', colors: oceanColors },
  { id: 'forest', name: '포레스트', colors: forestColors },
  { id: 'sunset', name: '선셋', colors: sunsetColors },
];

export function getDefaultColors(): ThemeColors {
  return { ...defaultColors };
}

export function applyTheme(colors: ThemeColors): void {
  const root = document.documentElement;
  root.style.setProperty('--color-paper', colors.paper);
  root.style.setProperty('--color-canvas', colors.canvas);
  root.style.setProperty('--color-mist', colors.mist);
  root.style.setProperty('--color-stroke', colors.stroke);
  root.style.setProperty('--color-ink', colors.ink);
  root.style.setProperty('--color-ink-muted', colors.inkMuted);
  root.style.setProperty('--color-accent-sky', colors.accentSky);
  root.style.setProperty('--color-accent-sky-strong', colors.accentSkyStrong);
  root.style.setProperty('--color-accent-mint', colors.accentMint);
  root.style.setProperty('--color-accent-mint-strong', colors.accentMintStrong);
  root.style.setProperty('--color-accent-peach', colors.accentPeach);
  root.style.setProperty('--color-accent-peach-strong', colors.accentPeachStrong);
  root.style.setProperty('--color-white', colors.white);
  root.style.setProperty('--color-border', colors.border);
}

export interface SavedTheme {
  presetId: string | null;
  customColors: ThemeColors | null;
}

export async function saveTheme(theme: SavedTheme): Promise<void> {
  try {
    await setSetting('theme', JSON.stringify(theme));
    try {
      await widgetApi.refreshWidgetCache();
    } catch (error) {
      console.error('Failed to refresh widget cache after theme change:', error);
    }
  } catch (error) {
    console.error('Failed to save theme:', error);
  }
}

export async function loadSavedTheme(): Promise<SavedTheme | null> {
  try {
    const value = await getSetting('theme');
    if (value) {
      return JSON.parse(value);
    }
  } catch (error) {
    console.error('Failed to load theme:', error);
  }
  return null;
}

export function getThemeColors(savedTheme: SavedTheme | null): ThemeColors {
  if (!savedTheme) {
    return getDefaultColors();
  }

  if (savedTheme.customColors) {
    return savedTheme.customColors;
  }

  if (savedTheme.presetId) {
    const preset = themePresets.find(p => p.id === savedTheme.presetId);
    if (preset) {
      return { ...preset.colors };
    }
  }

  return getDefaultColors();
}

export async function initializeTheme(): Promise<void> {
  const savedTheme = await loadSavedTheme();
  const colors = getThemeColors(savedTheme);
  applyTheme(colors);
}
