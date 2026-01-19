export interface Category {
  id: number;
  name: string;
  display_order: number;
}

export interface TodoItem {
  id: number;
  text: string;
  done: boolean;
  category_id: number | null;
  display_order: number;
  memo: string | null;
}
