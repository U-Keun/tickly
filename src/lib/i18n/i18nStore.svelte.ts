import { ko, type Translations } from './ko';
import { en } from './en';
import * as settingsApi from '../api/settingsApi';

export type Locale = 'ko' | 'en';

const translations: Record<Locale, Translations> = { ko, en };

let currentLocale = $state<Locale>('ko');

async function loadLocale(): Promise<void> {
  try {
    const saved = await settingsApi.getSetting('locale');
    if (saved === 'en' || saved === 'ko') {
      currentLocale = saved;
    }
  } catch (error) {
    console.error('Failed to load locale:', error);
  }
}

async function setLocale(locale: Locale): Promise<void> {
  currentLocale = locale;
  try {
    await settingsApi.setSetting('locale', locale);
  } catch (error) {
    console.error('Failed to save locale:', error);
  }
}

function t<K extends keyof Translations>(key: K): Translations[K] {
  return translations[currentLocale][key];
}

export const i18n = {
  get locale() {
    return currentLocale;
  },
  get t() {
    return t;
  },
  loadLocale,
  setLocale,
};
