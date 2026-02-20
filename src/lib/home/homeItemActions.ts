import type { RepeatType, TodoItem } from '../../types';

interface HomeItemActionDeps {
  addItem: (
    text: string,
    memo: string | null,
    repeatType: RepeatType,
    repeatDetail: string | null,
    trackStreak: boolean,
    tagNames?: string[],
    reminderAt?: string | null,
    linkedApp?: string | null
  ) => Promise<void>;
  updateReminder: (id: number, reminderAt: string | null) => Promise<void>;
  toggleItem: (id: number) => Promise<void>;
  deleteItem: (id: number) => Promise<void>;
  getItems: () => TodoItem[];
  scheduleReminder: (itemId: number, itemText: string, reminderAt: string) => Promise<void>;
  cancelReminder: (itemId: number) => Promise<void>;
}

export function createHomeItemActions(deps: HomeItemActionDeps) {
  async function handleAddItem(
    text: string,
    memo: string | null,
    repeatType: RepeatType,
    repeatDetail: string | null,
    trackStreak: boolean,
    tagNames?: string[],
    reminderAt?: string | null,
    linkedApp?: string | null
  ): Promise<void> {
    await deps.addItem(
      text,
      memo,
      repeatType,
      repeatDetail,
      trackStreak,
      tagNames ?? [],
      reminderAt ?? null,
      linkedApp ?? null
    );

    if (!reminderAt) return;

    const items = deps.getItems();
    const lastItem = items[items.length - 1];
    if (!lastItem) return;

    await deps.scheduleReminder(lastItem.id, lastItem.text, reminderAt);
  }

  async function handleUpdateReminder(id: number, reminderAt: string | null): Promise<void> {
    await deps.updateReminder(id, reminderAt);

    if (!reminderAt) {
      await deps.cancelReminder(id);
      return;
    }

    const item = deps.getItems().find((currentItem) => currentItem.id === id);
    await deps.scheduleReminder(id, item?.text ?? '', reminderAt);
  }

  async function handleToggleItem(id: number): Promise<void> {
    const item = deps.getItems().find((currentItem) => currentItem.id === id);
    const wasDone = item?.done;

    await deps.toggleItem(id);

    if (!wasDone && item?.reminder_at) {
      await deps.cancelReminder(id);
      return;
    }

    if (wasDone && item?.reminder_at) {
      await deps.scheduleReminder(id, item.text, item.reminder_at);
    }
  }

  async function handleDeleteItem(id: number): Promise<void> {
    await deps.cancelReminder(id);
    await deps.deleteItem(id);
  }

  return {
    handleAddItem,
    handleUpdateReminder,
    handleToggleItem,
    handleDeleteItem,
  };
}
