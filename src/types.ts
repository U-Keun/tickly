export interface Category {
  id: number;
  name: string;
  display_order: number;
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
