import { i18n } from '$lib/i18n';

export function formatLastSyncedAt(lastSyncedAt: string | null): string {
  if (!lastSyncedAt) {
    return '';
  }

  try {
    const date = new Date(lastSyncedAt);
    const now = new Date();
    const diffMs = now.getTime() - date.getTime();
    const diffMins = Math.floor(diffMs / 60000);

    if (diffMins < 1) {
      return i18n.t('justNow');
    } else if (diffMins < 60) {
      return i18n.t('minutesAgo')(diffMins);
    } else if (diffMins < 1440) {
      return i18n.t('hoursAgo')(Math.floor(diffMins / 60));
    } else {
      return date.toLocaleDateString();
    }
  } catch {
    return lastSyncedAt;
  }
}
