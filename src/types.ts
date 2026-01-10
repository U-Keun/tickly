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
