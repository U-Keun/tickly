interface HomeResetSchedulerDeps {
  getResetTime: () => Promise<string | null>;
  onReset: () => Promise<void>;
}

export function createHomeResetScheduler(deps: HomeResetSchedulerDeps) {
  let resetTimer: ReturnType<typeof setTimeout> | null = null;

  function getMsUntilResetTime(resetTime: string): number {
    const [hours, minutes] = resetTime.split(':').map(Number);
    const now = new Date();
    const resetDate = new Date();
    resetDate.setHours(hours, minutes, 0, 0);

    if (resetDate <= now) {
      resetDate.setDate(resetDate.getDate() + 1);
    }

    return resetDate.getTime() - now.getTime();
  }

  function clearResetTimer(): void {
    if (!resetTimer) {
      return;
    }

    clearTimeout(resetTimer);
    resetTimer = null;
  }

  async function scheduleResetTimer(): Promise<void> {
    clearResetTimer();

    try {
      const resetTime = await deps.getResetTime();
      if (!resetTime) {
        return;
      }

      const msUntilReset = getMsUntilResetTime(resetTime);
      resetTimer = setTimeout(async () => {
        await deps.onReset();
        await scheduleResetTimer();
      }, msUntilReset);
    } catch (error) {
      console.error('Failed to schedule reset timer:', error);
    }
  }

  return {
    clearResetTimer,
    scheduleResetTimer
  };
}
