import { invoke } from './client';
import type { RealtimeStatus } from '../../types';

/**
 * Connect to Supabase Realtime
 */
export async function connectRealtime(
  supabaseUrl: string,
  anonKey: string,
  accessToken: string,
  userId: string
): Promise<void> {
  return invoke<void>('connect_realtime', {
    supabaseUrl,
    anonKey,
    accessToken,
    userId,
  });
}

/**
 * Disconnect from Supabase Realtime
 */
export async function disconnectRealtime(): Promise<void> {
  return invoke<void>('disconnect_realtime');
}

/**
 * Get realtime connection status
 */
export async function getRealtimeStatus(): Promise<RealtimeStatus> {
  return invoke<RealtimeStatus>('get_realtime_status');
}

/**
 * Check if realtime is connected
 */
export async function isRealtimeConnected(): Promise<boolean> {
  return invoke<boolean>('is_realtime_connected');
}
