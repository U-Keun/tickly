import { invoke } from '@tauri-apps/api/core';
import type { ThemeColors, ThemePreset } from '../types';

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
  accentSky: '#60a5fa',
  accentSkyStrong: '#3b82f6',
  accentMint: '#34d399',
  accentMintStrong: '#10b981',
  accentPeach: '#fbbf24',
  accentPeachStrong: '#f59e0b',
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
  accentMint: '#67e8f9',
  accentMintStrong: '#22d3ee',
  accentPeach: '#fcd34d',
  accentPeachStrong: '#fbbf24',
  white: '#f0f9ff',
  border: '#7dd3fc',
};

// Forest theme (포레스트)
const forestColors: ThemeColors = {
  paper: '#ecfdf5',
  canvas: '#d1fae5',
  mist: '#a7f3d0',
  stroke: '#6ee7b7',
  ink: '#064e3b',
  inkMuted: '#065f46',
  accentSky: '#34d399',
  accentSkyStrong: '#10b981',
  accentMint: '#6ee7b7',
  accentMintStrong: '#34d399',
  accentPeach: '#fcd34d',
  accentPeachStrong: '#fbbf24',
  white: '#f0fdf4',
  border: '#a7f3d0',
};

// Sunset theme (선셋)
const sunsetColors: ThemeColors = {
  paper: '#fef3c7',
  canvas: '#fde68a',
  mist: '#fcd34d',
  stroke: '#fbbf24',
  ink: '#78350f',
  inkMuted: '#92400e',
  accentSky: '#f97316',
  accentSkyStrong: '#ea580c',
  accentMint: '#fb923c',
  accentMintStrong: '#f97316',
  accentPeach: '#ef4444',
  accentPeachStrong: '#dc2626',
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
    await invoke('set_setting', { key: 'theme', value: JSON.stringify(theme) });
  } catch (error) {
    console.error('Failed to save theme:', error);
  }
}

export async function loadSavedTheme(): Promise<SavedTheme | null> {
  try {
    const value = await invoke<string | null>('get_setting', { key: 'theme' });
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
