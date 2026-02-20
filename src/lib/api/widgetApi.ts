import { invoke } from './client';
import type { WidgetSnapshot } from '../../types';

export async function getWidgetSnapshot(maxItems?: number): Promise<WidgetSnapshot> {
  return invoke<WidgetSnapshot>('get_widget_snapshot', {
    maxItems: maxItems ?? null,
  });
}

export async function refreshWidgetCache(maxItems?: number): Promise<WidgetSnapshot> {
  return invoke<WidgetSnapshot>('refresh_widget_cache', {
    maxItems: maxItems ?? null,
  });
}

export async function toggleItemFromWidget(id: number, maxItems?: number): Promise<WidgetSnapshot> {
  return invoke<WidgetSnapshot>('toggle_item_from_widget', {
    id,
    maxItems: maxItems ?? null,
  });
}

export async function processWidgetActions(maxItems?: number): Promise<number> {
  return invoke<number>('process_widget_actions', {
    maxItems: maxItems ?? null,
  });
}

export async function setWidgetCachePath(path: string): Promise<void> {
  return invoke<void>('set_widget_cache_path', { path });
}

export async function getWidgetCachePath(): Promise<string> {
  return invoke<string>('get_widget_cache_path');
}

export async function setWidgetAppGroupId(appGroupId: string): Promise<void> {
  return invoke<void>('set_widget_app_group_id', { appGroupId });
}

export async function getWidgetAppGroupId(): Promise<string> {
  return invoke<string>('get_widget_app_group_id');
}
