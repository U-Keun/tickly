import { invokeTauri } from './tauriClient';

export async function getSetting(key: string): Promise<string | null> {
  return invokeTauri<string | null>('get_setting', { key });
}

export async function setSetting(key: string, value: string): Promise<void> {
  await invokeTauri('set_setting', { key, value });
}
