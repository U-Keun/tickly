import { openUrl } from '@tauri-apps/plugin-opener';
import * as settingsApi from '$lib/api/settingsApi';

export interface LinkedApp {
  key: string;
  label: { ko: string; en: string; ja: string };
  urlScheme: string;
  icon: string;
}

export interface CustomApp {
  key: string;
  name: string;
  urlScheme: string;
}

const SETTINGS_KEY = 'custom_linked_apps';

export const PRESET_APPS: LinkedApp[] = [
  { key: 'weather', label: { ko: '날씨', en: 'Weather', ja: '天気' }, urlScheme: 'weather://open', icon: '🌤' },
  { key: 'calendar', label: { ko: '캘린더', en: 'Calendar', ja: 'カレンダー' }, urlScheme: 'calshow://today', icon: '📅' },
  { key: 'notes', label: { ko: '메모', en: 'Notes', ja: 'メモ' }, urlScheme: 'mobilenotes://open', icon: '📝' },
  { key: 'reminders', label: { ko: '리마인더', en: 'Reminders', ja: 'リマインダー' }, urlScheme: 'x-apple-reminderkit://open', icon: '✅' },
  { key: 'safari', label: { ko: 'Safari', en: 'Safari', ja: 'Safari' }, urlScheme: 'https://apple.com', icon: '🌐' },
  { key: 'mail', label: { ko: '메일', en: 'Mail', ja: 'メール' }, urlScheme: 'mailto:?subject=', icon: '📧' },
  { key: 'maps', label: { ko: '지도', en: 'Maps', ja: 'マップ' }, urlScheme: 'maps://maps.apple.com', icon: '📍' },
  { key: 'health', label: { ko: '건강', en: 'Health', ja: 'ヘルスケア' }, urlScheme: 'x-apple-health://open', icon: '❤️' },
];

let customAppsCache: CustomApp[] = [];

export async function loadCustomApps(): Promise<CustomApp[]> {
  try {
    const json = await settingsApi.getSetting(SETTINGS_KEY);
    customAppsCache = json ? JSON.parse(json) : [];
  } catch {
    customAppsCache = [];
  }
  return customAppsCache;
}

export function getCustomApps(): CustomApp[] {
  return customAppsCache;
}

async function saveCustomApps(apps: CustomApp[]): Promise<void> {
  customAppsCache = apps;
  await settingsApi.setSetting(SETTINGS_KEY, JSON.stringify(apps));
}

export async function addCustomApp(name: string, urlScheme: string): Promise<CustomApp> {
  const key = 'custom_' + name.toLowerCase().replace(/[^a-z0-9]/g, '_') + '_' + Date.now();
  const app: CustomApp = { key, name, urlScheme };
  await saveCustomApps([...customAppsCache, app]);
  return app;
}

export async function removeCustomApp(key: string): Promise<void> {
  await saveCustomApps(customAppsCache.filter(a => a.key !== key));
}

export function getAppByKey(key: string): LinkedApp | { key: string; icon: string; label: { ko: string; en: string; ja: string }; urlScheme: string } | undefined {
  const preset = PRESET_APPS.find(app => app.key === key);
  if (preset) return preset;
  const custom = customAppsCache.find(a => a.key === key);
  if (custom) {
    return {
      key: custom.key,
      icon: '🔗',
      label: { ko: custom.name, en: custom.name, ja: custom.name },
      urlScheme: custom.urlScheme,
    };
  }
  return undefined;
}

export function getAppLabel(key: string, locale: string): string {
  const app = getAppByKey(key);
  if (!app) return key;
  return app.label[locale as keyof typeof app.label] || app.label.en;
}

export async function openApp(key: string): Promise<void> {
  const app = getAppByKey(key);
  if (!app) return;
  try {
    await openUrl(app.urlScheme);
  } catch (error) {
    console.error('Failed to open app:', error);
  }
}
