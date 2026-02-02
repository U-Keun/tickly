import * as syncApi from '$lib/api/syncApi';
import * as realtimeApi from '$lib/api/realtimeApi';
import type {
  SyncResult,
  SyncStatusInfo,
  RealtimeConnectionState,
  RealtimeEvent,
  DataChangedEvent,
} from '../../types';
import { appStore } from './appStore.svelte';

// Supabase config - loaded from environment at build time
const SUPABASE_URL = import.meta.env.VITE_SUPABASE_URL || '';
const SUPABASE_ANON_KEY = import.meta.env.VITE_SUPABASE_ANON_KEY || '';

// Debounce delay for auto-sync (in milliseconds)
const SYNC_DEBOUNCE_MS = 2000;

class SyncStore {
  // Manual sync state
  isEnabled = $state(false);
  isSyncing = $state(false);
  isOnline = $state(false);
  lastSyncedAt = $state<string | null>(null);
  pendingCount = $state(0);
  lastSyncResult = $state<SyncResult | null>(null);
  error = $state<string | null>(null);

  // Realtime state
  realtimeState = $state<RealtimeConnectionState>('disconnected');
  realtimeError = $state<string | null>(null);
  private realtimeUnlisten: (() => void) | null = null;
  private dataChangedUnlisten: (() => void) | null = null;

  // Debounced sync timer
  private syncDebounceTimer: ReturnType<typeof setTimeout> | null = null;

  get isRealtimeConnected(): boolean {
    return this.realtimeState === 'connected';
  }

  // Setter methods for external callback reactivity
  private setRealtimeState(state: RealtimeConnectionState) {
    this.realtimeState = state;
  }

  private setRealtimeError(error: string | null) {
    this.realtimeError = error;
  }

  async loadStatus(): Promise<void> {
    try {
      const status = await syncApi.getSyncStatus();
      this.isEnabled = status.is_enabled;
      this.isSyncing = status.is_syncing;
      this.isOnline = status.is_online;
      this.lastSyncedAt = status.last_synced_at;
      this.pendingCount = status.pending_count;
      this.error = null;

      // Also load realtime status
      const realtimeStatus = await realtimeApi.getRealtimeStatus();
      this.setRealtimeState(realtimeStatus.state);
      this.setRealtimeError(realtimeStatus.last_error ?? null);
    } catch (error) {
      console.error('Failed to load sync status:', error);
      this.error = error instanceof Error ? error.message : String(error);
    }
  }

  async sync(): Promise<SyncResult | null> {
    if (this.isSyncing) {
      return null;
    }

    this.isSyncing = true;
    this.error = null;

    try {
      const result = await syncApi.triggerSync();
      this.lastSyncResult = result;
      this.lastSyncedAt = result.last_synced_at;
      this.pendingCount = 0;

      // Always refresh app data after sync to reflect any changes
      await appStore.refreshAll();

      return result;
    } catch (error) {
      console.error('Sync failed:', error);
      this.error = error instanceof Error ? error.message : String(error);
      return null;
    } finally {
      this.isSyncing = false;
    }
  }

  async setEnabled(enabled: boolean): Promise<void> {
    try {
      await syncApi.setSyncEnabled(enabled);
      this.isEnabled = enabled;
    } catch (error) {
      console.error('Failed to set sync enabled:', error);
      throw error;
    }
  }

  async refreshPendingCount(): Promise<void> {
    try {
      this.pendingCount = await syncApi.getPendingCount();
    } catch (error) {
      console.error('Failed to get pending count:', error);
    }
  }

  /**
   * Schedule a debounced sync.
   * Call this when local data changes to auto-sync after a delay.
   */
  scheduleSync(): void {
    // Only schedule if sync is enabled and realtime is connected
    if (!this.isEnabled || !this.isRealtimeConnected) {
      return;
    }

    // Clear existing timer
    if (this.syncDebounceTimer) {
      clearTimeout(this.syncDebounceTimer);
    }

    // Schedule new sync
    this.syncDebounceTimer = setTimeout(async () => {
      this.syncDebounceTimer = null;
      await this.sync();
    }, SYNC_DEBOUNCE_MS);
  }

  // ===== Realtime Methods =====

  /**
   * Connect to Supabase Realtime
   */
  async connectRealtime(accessToken: string, userId: string): Promise<void> {
    if (!SUPABASE_URL || !SUPABASE_ANON_KEY) {
      return;
    }

    try {
      // Set up event listeners first
      await this.setupRealtimeListeners();

      // Connect
      this.setRealtimeState('connecting');
      await realtimeApi.connectRealtime(SUPABASE_URL, SUPABASE_ANON_KEY, accessToken, userId);
    } catch (error) {
      console.error('Failed to connect realtime:', error);
      this.setRealtimeError(error instanceof Error ? error.message : String(error));
      this.setRealtimeState('disconnected');
    }
  }

  /**
   * Disconnect from Supabase Realtime
   */
  async disconnectRealtime(): Promise<void> {
    try {
      await realtimeApi.disconnectRealtime();
      this.setRealtimeState('disconnected');
      this.setRealtimeError(null);

      // Clean up listeners
      this.cleanupRealtimeListeners();
    } catch (error) {
      console.error('Failed to disconnect realtime:', error);
    }
  }

  /**
   * Set up Tauri event listeners for realtime events
   */
  private async setupRealtimeListeners(): Promise<void> {
    // Clean up existing listeners
    this.cleanupRealtimeListeners();

    try {
      const { listen } = await import('@tauri-apps/api/event');

      // Listen for realtime connection events
      this.realtimeUnlisten = await listen<RealtimeEvent>('realtime-event', (event) => {
        const { event_type, message } = event.payload;

        switch (event_type) {
          case 'connected':
            this.setRealtimeState('connected');
            this.setRealtimeError(null);
            break;
          case 'disconnected':
            this.setRealtimeState('disconnected');
            break;
          case 'reconnecting':
            this.setRealtimeState('reconnecting');
            break;
          case 'error':
            this.setRealtimeError(message ?? null);
            break;
        }
      });

      // Listen for data change events
      this.dataChangedUnlisten = await listen<DataChangedEvent>('data-changed', async (event) => {
        const { table, change_type } = event.payload;
        await this.handleDataChange(table, change_type);
      });
    } catch (error) {
      console.error('Failed to set up realtime listeners:', error);
    }
  }

  /**
   * Handle data change from realtime subscription
   */
  private async handleDataChange(table: string, changeType: string): Promise<void> {
    try {
      // Trigger sync to pull latest data from server
      // This updates the local DB, then refreshes the UI
      await this.sync();
    } catch (error) {
      console.error('Failed to handle data change:', error);
    }
  }

  /**
   * Clean up realtime event listeners
   */
  private cleanupRealtimeListeners(): void {
    if (this.realtimeUnlisten) {
      this.realtimeUnlisten();
      this.realtimeUnlisten = null;
    }
    if (this.dataChangedUnlisten) {
      this.dataChangedUnlisten();
      this.dataChangedUnlisten = null;
    }
  }

  formatLastSyncedAt(): string {
    if (!this.lastSyncedAt) {
      return '';
    }

    try {
      const date = new Date(this.lastSyncedAt);
      const now = new Date();
      const diffMs = now.getTime() - date.getTime();
      const diffMins = Math.floor(diffMs / 60000);

      if (diffMins < 1) {
        return '방금 전';
      } else if (diffMins < 60) {
        return `${diffMins}분 전`;
      } else if (diffMins < 1440) {
        return `${Math.floor(diffMins / 60)}시간 전`;
      } else {
        return date.toLocaleDateString();
      }
    } catch {
      return this.lastSyncedAt;
    }
  }
}

export const syncStore = new SyncStore();
