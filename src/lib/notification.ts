import { invoke } from '@tauri-apps/api/core';
import type { TodoItem } from '../types';

/**
 * Check permission via native Tauri invoke (NOT window.Notification which doesn't work on iOS).
 */
async function nativeIsPermissionGranted(): Promise<boolean> {
  return await invoke<boolean>('plugin:notification|is_permission_granted');
}

/**
 * Request permission via native Tauri invoke.
 */
async function nativeRequestPermission(): Promise<string> {
  return await invoke<string>('plugin:notification|request_permission');
}

/**
 * Send/schedule notification via native Tauri invoke.
 * The plugin's exported sendNotification uses window.Notification (Web API)
 * which doesn't support scheduling and doesn't work on iOS WKWebView.
 */
async function nativeSendNotification(options: Record<string, unknown>): Promise<void> {
  await invoke('plugin:notification|notify', { options });
}

/**
 * Get pending scheduled notifications via native Tauri invoke.
 */
async function nativeGetPending(): Promise<{ id: number }[]> {
  return await invoke('plugin:notification|get_pending');
}

/**
 * Cancel notifications by IDs via native Tauri invoke.
 */
async function nativeCancel(ids: number[]): Promise<void> {
  await invoke('plugin:notification|cancel', { notifications: ids });
}

/**
 * Request notification permission if not already granted.
 * Returns true if permission is granted.
 */
export async function ensurePermission(): Promise<boolean> {
  try {
    let granted = await nativeIsPermissionGranted();
    if (!granted) {
      const result = await nativeRequestPermission();
      granted = result === 'granted';
    }
    return granted;
  } catch (e) {
    console.error('Notification permission error:', e);
    return false;
  }
}

/**
 * Generate a stable numeric ID from item ID.
 * Notification IDs must be 32-bit integers.
 */
function notificationId(itemId: number): number {
  return itemId;
}

/**
 * Schedule a reminder notification for an item.
 *
 * Uses Schedule.interval (UNCalendarNotificationTrigger) instead of Schedule.at
 * because the Rust plugin re-serializes the Date with time::iso8601 (e.g. "+00")
 * which doesn't match the Swift DateFormatter expected format ("Z" + ".SSS").
 * interval only passes integer hour/minute values — no date string round-trip issues.
 *
 * @param itemId - The todo item ID
 * @param text - The notification body text (item text)
 * @param timeStr - Time in "HH:MM" format
 */
export async function scheduleReminder(
  itemId: number,
  text: string,
  timeStr: string
): Promise<void> {
  try {
    const granted = await ensurePermission();
    if (!granted) return;

    // Cancel any existing reminder for this item first
    await cancelReminder(itemId);

    const [hours, minutes] = timeStr.split(':').map(Number);

    // Build schedule using "interval" variant (maps to UNCalendarNotificationTrigger).
    // This fires daily at the specified hour:minute and repeats until cancelled.
    const schedule = {
      interval: {
        interval: { hour: hours, minute: minutes },
        allowWhileIdle: true,
      },
    };

    await nativeSendNotification({
      id: notificationId(itemId),
      title: 'Tickly',
      body: text,
      schedule,
    });
  } catch (e) {
    console.error('Failed to schedule reminder:', e);
  }
}

/**
 * Cancel a scheduled reminder for an item.
 */
export async function cancelReminder(itemId: number): Promise<void> {
  try {
    const pendingList = await nativeGetPending();
    const id = notificationId(itemId);
    if (pendingList.some((n) => n.id === id)) {
      await nativeCancel([id]);
    }
  } catch (e) {
    console.error('Failed to cancel reminder:', e);
  }
}

/**
 * Reschedule all reminders on app startup.
 */
export async function rescheduleAll(items: TodoItem[]): Promise<void> {
  try {
    const granted = await ensurePermission();
    if (!granted) return;

    for (const item of items) {
      if (item.reminder_at && !item.done) {
        await scheduleReminder(item.id, item.text, item.reminder_at);
      }
    }
  } catch (e) {
    console.error('Failed to reschedule reminders:', e);
  }
}
