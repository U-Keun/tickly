import { invoke } from '@tauri-apps/api/core';

type InvokePayload = Record<string, unknown> | undefined;

export async function invokeTauri<T>(command: string, payload?: InvokePayload): Promise<T> {
  try {
    return await invoke<T>(command, payload);
  } catch (error) {
    console.error(`Tauri invoke failed: ${command}`, error);
    throw error;
  }
}
