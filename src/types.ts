export interface Category {
  id: number;
  name: string;
}

export interface TodoItem {
  id: number;
  text: string;
  done: boolean;
  category_id: number | null;
}

export interface Template {
  id: number;
  text: string;
  category_id: number | null;
  display_order: number;
}
