import type { DataChangedEvent, RealtimeConnectionState, RealtimeEvent } from '../../types';
import * as realtimeApi from '$lib/api/realtimeApi';

interface RealtimeBridgeContext {
  supabaseUrl: string;
  supabaseAnonKey: string;
  setRealtimeState: (state: RealtimeConnectionState) => void;
  setRealtimeError: (error: string | null) => void;
  sync: () => Promise<void>;
  getErrorMessage: (error: unknown) => string;
}

export class RealtimeBridge {
  private realtimeUnlisten: (() => void) | null = null;
  private dataChangedUnlisten: (() => void) | null = null;

  constructor(private readonly context: RealtimeBridgeContext) {}

  async getStatus() {
    return realtimeApi.getRealtimeStatus();
  }

  async connect(accessToken: string, userId: string): Promise<void> {
    if (!this.context.supabaseUrl || !this.context.supabaseAnonKey) {
      return;
    }

    try {
      await this.setupRealtimeListeners();
      this.context.setRealtimeState('connecting');
      await realtimeApi.connectRealtime(
        this.context.supabaseUrl,
        this.context.supabaseAnonKey,
        accessToken,
        userId
      );
    } catch (error) {
      console.error('Failed to connect realtime:', error);
      this.context.setRealtimeError(this.context.getErrorMessage(error));
      this.context.setRealtimeState('disconnected');
    }
  }

  async disconnect(): Promise<void> {
    try {
      await realtimeApi.disconnectRealtime();
      this.context.setRealtimeState('disconnected');
      this.context.setRealtimeError(null);
      this.cleanupRealtimeListeners();
    } catch (error) {
      console.error('Failed to disconnect realtime:', error);
    }
  }

  private async setupRealtimeListeners(): Promise<void> {
    this.cleanupRealtimeListeners();

    try {
      const { listen } = await import('@tauri-apps/api/event');

      this.realtimeUnlisten = await listen<RealtimeEvent>('realtime-event', (event) => {
        const { event_type, message } = event.payload;

        switch (event_type) {
          case 'connected':
            this.context.setRealtimeState('connected');
            this.context.setRealtimeError(null);
            break;
          case 'disconnected':
            this.context.setRealtimeState('disconnected');
            break;
          case 'reconnecting':
            this.context.setRealtimeState('reconnecting');
            break;
          case 'error':
            this.context.setRealtimeError(message ?? null);
            break;
        }
      });

      this.dataChangedUnlisten = await listen<DataChangedEvent>('data-changed', async () => {
        await this.handleDataChange();
      });
    } catch (error) {
      console.error('Failed to set up realtime listeners:', error);
    }
  }

  private async handleDataChange(): Promise<void> {
    try {
      await this.context.sync();
    } catch (error) {
      console.error('Failed to handle data change:', error);
    }
  }

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
}
