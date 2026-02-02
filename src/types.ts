export type SyncStatus = 'pending' | 'synced' | 'conflict' | 'deleted';

export interface Category {
  id: number;
  name: string;
  display_order: number;
  // Sync fields (optional for dnd library compatibility)
  sync_id?: string | null;
  created_at?: string | null;
  updated_at?: string | null;
  sync_status?: SyncStatus;
}

export type RepeatType = 'none' | 'daily' | 'weekly' | 'monthly';

export interface TodoItem {
  id: number;
  text: string;
  done: boolean;
  category_id: number | null;
  display_order: number;
  memo: string | null;
  repeat_type: RepeatType;
  repeat_detail: string | null;
  next_due_at: string | null;
  last_completed_at: string | null;
  track_streak: boolean;
  // Sync fields (optional for dnd library compatibility)
  sync_id?: string | null;
  created_at?: string | null;
  updated_at?: string | null;
  sync_status?: SyncStatus;
}

export interface ThemeColors {
  paper: string;
  canvas: string;
  mist: string;
  stroke: string;
  ink: string;
  inkMuted: string;
  accentSky: string;
  accentSkyStrong: string;
  accentMint: string;
  accentMintStrong: string;
  accentPeach: string;
  accentPeachStrong: string;
  white: string;
  border: string;
}

export interface ThemePreset {
  id: string;
  name: string;
  colors: ThemeColors;
}

export type FontSize = 'small' | 'medium' | 'large';

export interface FontPreset {
  id: string;
  name: string;
  fontFamily: string;
}

export interface FontSettings {
  presetId: string;
  size: FontSize;
}

export interface CompletionLog {
  item_id: number;
  completed_on: string;
  completed_count: number;
}

export interface HeatmapData {
  item_id: number;
  item_text: string;
  logs: CompletionLog[];
  total_days: number;
  current_streak: number;
  longest_streak: number;
}

export interface TrackedItem {
  id: number;
  text: string;
  category_id: number | null;
}

// Auth types
export type AuthProvider = 'apple' | 'google';

export interface AuthSession {
  user_id: string;
  access_token: string;
  refresh_token: string;
  expires_at: string;
  provider: AuthProvider;
}

export interface UserProfile {
  id: string;
  email?: string;
  full_name?: string;
  avatar_url?: string;
}

// Sync types
export interface SyncResult {
  pushed: number;
  pulled: number;
  conflicts: number;
  last_synced_at: string | null;
}

export interface SyncStatusInfo {
  is_enabled: boolean;
  is_syncing: boolean;
  is_online: boolean;
  pending_count: number;
  last_synced_at: string | null;
}

export interface ConflictItem {
  local_id: number;
  sync_id: string;
  local_text: string;
  remote_text: string;
  local_updated_at: string;
  remote_updated_at: string;
}

export interface Template {
  id: number;
  text: string;
}

// Realtime types
export type RealtimeConnectionState =
  | 'disconnected'
  | 'connecting'
  | 'connected'
  | 'reconnecting';

export interface RealtimeStatus {
  state: RealtimeConnectionState;
  reconnect_attempts: number;
  last_error: string | null;
}

export type RealtimeEventType = 'connected' | 'disconnected' | 'reconnecting' | 'error';

export interface RealtimeEvent {
  event_type: RealtimeEventType;
  message: string | null;
}

export type DataChangeType = 'INSERT' | 'UPDATE' | 'DELETE';

export interface DataChangedEvent {
  table: string;
  change_type: DataChangeType;
  sync_id: string | null;
}
