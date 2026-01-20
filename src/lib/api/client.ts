import { invoke as tauriInvoke } from '@tauri-apps/api/core';

/**
 * Wrapper around Tauri invoke with consistent error handling
 */
export async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  try {
    return await tauriInvoke<T>(cmd, args);
  } catch (error) {
    console.error(`Tauri command "${cmd}" failed:`, error);
    throw error;
  }
}
