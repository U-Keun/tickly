import * as syncApi from '$lib/api/syncApi';
import type { SyncResult, SyncStatusInfo } from '../../types';
import { appStore } from './appStore.svelte';

class SyncStore {
  isEnabled = $state(false);
  isSyncing = $state(false);
  isOnline = $state(false);
  lastSyncedAt = $state<string | null>(null);
  pendingCount = $state(0);
  lastSyncResult = $state<SyncResult | null>(null);
  error = $state<string | null>(null);

  async loadStatus(): Promise<void> {
    try {
      const status = await syncApi.getSyncStatus();
      this.isEnabled = status.is_enabled;
      this.isSyncing = status.is_syncing;
      this.isOnline = status.is_online;
      this.lastSyncedAt = status.last_synced_at;
      this.pendingCount = status.pending_count;
      this.error = null;
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
