import type { HeatmapData, TrackedItem } from '../../types';
import { invoke } from './client';

/**
 * Get all tracked items
 */
export async function getTrackedItems(): Promise<TrackedItem[]> {
  return invoke<TrackedItem[]>('get_tracked_items');
}

/**
 * Get heatmap data for a specific item
 */
export async function getItemHeatmapData(itemId: number): Promise<HeatmapData | null> {
  return invoke<HeatmapData | null>('get_item_heatmap_data', { itemId });
}

/**
 * Update track_streak setting for an item
 */
export async function updateTrackStreak(id: number, trackStreak: boolean): Promise<void> {
  return invoke<void>('update_track_streak', { id, trackStreak });
}
